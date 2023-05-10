use std::{fmt::Debug, fs::File, io::Write};

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};
use piston::window::WindowSettings;
use piston::{CloseArgs, EventSettings, Events, Input};
use scarab_engine::gameobject::entity::registry::RegisteredDebugEntity;
use scarab_engine::rendering::debug::DebugView;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use scarab_engine::{
    gameobject::entity::registry::RegisteredEntity,
    gameobject::Field,
    input::InputRegistry,
    rendering::registry::{TextureList, TextureRegistry},
    rendering::View,
    App, Camera, ScarabError, ScarabResult, Scene,
};

use crate::debug::DebugOptions;
use crate::external_serde::EventSettingsDef;

/// A semver-like version of the AppData's save format
static SAVE_VERSION: &'static str = "0.1.0";

pub struct ExampleApp<E, V, I> {
    gl: GlGraphics, // OpenGL drawing backend.
    window: Window,
    data: AppData<E, V, I>,
    save_name: String,
    texture_registry: TextureRegistry,
}

impl<E, V, I> ExampleApp<E, V, I> {
    pub fn new(
        gl: GlGraphics,
        window: Window,
        scene: Scene<E, V>,
        camera: Camera,
        input_registry: I,
        debug_options: DebugOptions,
        save_name: String,
        event_settings: EventSettings,
        texture_registry: TextureRegistry,
    ) -> ScarabResult<Self> {
        Ok(Self {
            gl,
            window,
            data: AppData {
                save_version: SAVE_VERSION.to_string(),
                scene,
                camera,
                input_registry,
                debug_options,
                event_settings,
                texture_list: (&texture_registry).into(),
            },
            save_name,
            texture_registry,
        })
    }
}

impl<'e, 's, E, V, I> ExampleApp<E, V, I>
where
    's: 'e,
    E: RegisteredEntity,
    E: DeserializeOwned,
    V: View<Viewed = Field>,
    V: DeserializeOwned,
    I: InputRegistry<InputTarget = E::Player<'e, 's>>,
    I: DeserializeOwned,
{
    pub fn load_from_save(opengl: OpenGL, save_name: String) -> ScarabResult<Self> {
        let window: Window = WindowSettings::new("scarab-example", [300, 400])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap(); // TODO: don't panic here

        let file = File::open(&save_name).map_err(|e| ScarabError::RawString(format!("{:}", e)))?;
        let app_data: AppData<E, V, I> = rmp_serde::from_read(file)
            .map_err(|e| ScarabError::RawString(format!("Could not parse file: {:?}", e)))?;

        // Lazy version checking requires exact match.
        if app_data.save_version != SAVE_VERSION {
            return Err(ScarabError::RawString(format!(
                "Save version mismatch: save has {:}, needs {SAVE_VERSION:}",
                app_data.save_version
            )));
        }

        let texture_registry = app_data.texture_list.clone().try_into()?;

        Ok(Self {
            gl: GlGraphics::new(opengl),
            window,
            data: app_data,
            save_name,
            texture_registry,
        })
    }
}

impl<'a, 'e, 's, E, V, I> App<'a, Window> for ExampleApp<E, V, I>
where
    'a: 's,
    's: 'e,
    E: RegisteredEntity + RegisteredDebugEntity<DebugOptions = DebugOptions> + Debug + 'static,
    E: Serialize,
    V: View<Viewed = Field> + DebugView<Viewed = Field, DebugOptions = DebugOptions>,
    V: Serialize,
    I: InputRegistry<InputTarget = E::Player<'e, 's>>,
    I: Serialize,
{
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const GREY: [f32; 4] = [0.4, 0.4, 0.4, 1.0];

        self.gl.draw(args.viewport(), |ctx, gl| {
            // Clear the screen.
            clear(GREY, gl);

            self.data
                .scene
                .render_with_info(
                    &self.data.debug_options,
                    args,
                    &self.data.camera,
                    ctx,
                    &self.texture_registry,
                    gl,
                )
                .unwrap();
            self.data.camera.render_gutters(BLACK, args, ctx, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let _ = self
            .data
            .scene
            .tick_entities(args.dt)
            .map_err(|e| println!("Ticking entities failed with error: {e:}"));
    }

    fn resize(&mut self, args: &piston::ResizeArgs) {
        self.data.camera.set_window_size(&args.window_size)
    }

    fn close(self: Box<Self>, _args: &CloseArgs) {
        let mut f = File::create(&self.save_name).unwrap();
        let mut buf = Vec::new();
        AppData::from(self)
            .serialize(&mut rmp_serde::Serializer::new(&mut buf))
            .unwrap();

        let _ = f
            .write_all(&buf)
            .map_err(|e| println!("Saving app state failed with error: {e}"));
    }

    fn window(&mut self) -> &mut Window {
        &mut self.window
    }

    fn input_event(&mut self, input: Input) {
        if let Some(action) = self.data.input_registry.map_input_to_action(input) {
            self.data.do_player_action(action);
        }
    }

    fn events(&self) -> piston::Events {
        Events::new(self.data.event_settings)
    }
}

/// All data about an app that should be savable between instances
/// i.e. scenes, controls, rendering settings
#[derive(Debug, Serialize, Deserialize)]
pub struct AppData<E, V, I> {
    save_version: String,
    scene: Scene<E, V>,
    camera: Camera,
    input_registry: I,
    debug_options: DebugOptions,
    #[serde(with = "EventSettingsDef")]
    event_settings: EventSettings,
    texture_list: TextureList,
}

impl<'e, 's, A, E, V, I> AppData<E, V, I>
where
    's: 'e,
    E: RegisteredEntity + Debug + 'static,
    V: View<Viewed = Field>,
    I: InputRegistry<InputActions = A, InputTarget = E::Player<'e, 's>>,
{
    fn do_player_action(&mut self, action: A) {
        if let Some(player) = self.scene.player_mut() {
            let _ = self
                .input_registry
                .do_input_action(action, player)
                .map_err(|e| println!("Doing input action failed with error: {e}"));
        } else {
            println!("Could not find player");
        }
    }
}

impl<'e, 's, E, V, I> From<ExampleApp<E, V, I>> for AppData<E, V, I>
where
    's: 'e,
    E: RegisteredEntity,
    V: View<Viewed = Field>,
    I: InputRegistry<InputTarget = E::Player<'e, 's>>,
{
    fn from(app: ExampleApp<E, V, I>) -> Self {
        let mut data = app.data;
        let new_texture_list = app.texture_registry.into();
        data.texture_list = new_texture_list;
        data
    }
}

impl<'e, 's, A, E, V, I> From<Box<ExampleApp<E, V, I>>> for AppData<E, V, I>
where
    's: 'e,
    E: RegisteredEntity,
    V: View<Viewed = Field>,
    I: InputRegistry<InputActions = A, InputTarget = E::Player<'e, 's>>,
{
    fn from(app: Box<ExampleApp<E, V, I>>) -> Self {
        app.data
    }
}
