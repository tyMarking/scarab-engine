use std::fs::File;
use std::io::Write;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};
use piston::window::WindowSettings;
use piston::{CloseArgs, EventSettings, Events, Input};
use scarab_engine::gameobject::Field;
use scarab_engine::rendering::registry::{TextureList, TextureRegistry};
use scarab_engine::rendering::View;
use scarab_engine::ScarabError;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use scarab_engine::{
    gameobject::entity::registry::RegisteredEntity, input::InputRegistry, App, Camera,
    ScarabResult, Scene,
};

use crate::external_serde::EventSettingsDef;

/// A semver-like version of the AppData's save format
static SAVE_VERSION: &'static str = "0.1.0";

pub struct ExampleApp<E, V, I: InputRegistry<InputTarget = E>> {
    gl: GlGraphics, // OpenGL drawing backend.
    window: Window,
    data: AppData<E, V, I>,
    save_name: String,
    texture_registry: TextureRegistry,
}

impl<E: RegisteredEntity, V: View<Viewed = Field>, I: InputRegistry<InputTarget = E>>
    ExampleApp<E, V, I>
{
    pub fn new(
        gl: GlGraphics,
        window: Window,
        scene: Scene<E, V>,
        camera: Camera,
        input_registry: I,
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
                event_settings,
                texture_list: (&texture_registry).into(),
            },
            save_name,
            texture_registry,
        })
    }
}

impl<
        E: RegisteredEntity + DeserializeOwned,
        V: View<Viewed = Field> + DeserializeOwned,
        I: InputRegistry<InputTarget = E> + DeserializeOwned,
    > ExampleApp<E, V, I>
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

impl<
        'a,
        E: RegisteredEntity + Serialize,
        V: View<Viewed = Field> + Serialize,
        I: InputRegistry<InputTarget = E> + Serialize,
    > App<'a, Window> for ExampleApp<E, V, I>
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
                .render(args, &self.data.camera, ctx, &self.texture_registry, gl)
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
pub struct AppData<E, V, I: InputRegistry> {
    save_version: String,
    scene: Scene<E, V>,
    camera: Camera,
    input_registry: I,
    #[serde(with = "EventSettingsDef")]
    event_settings: EventSettings,
    texture_list: TextureList,
}

impl<
        A,
        E: RegisteredEntity,
        V: View<Viewed = Field>,
        I: InputRegistry<InputActions = A, InputTarget = E>,
    > AppData<E, V, I>
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

impl<E, V, I: InputRegistry<InputTarget = E>> From<ExampleApp<E, V, I>> for AppData<E, V, I> {
    fn from(app: ExampleApp<E, V, I>) -> Self {
        let mut data = app.data;
        let new_texture_list = app.texture_registry.into();
        data.texture_list = new_texture_list;
        data
    }
}

impl<E, V, I: InputRegistry<InputTarget = E>> From<Box<ExampleApp<E, V, I>>> for AppData<E, V, I> {
    fn from(app: Box<ExampleApp<E, V, I>>) -> Self {
        app.data
    }
}
