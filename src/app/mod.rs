use std::sync::{Arc, RwLock};

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, ButtonArgs, ButtonEvent, ButtonState, CloseArgs, CloseEvent, Event, Key};

use crate::{Camera, Gamestate, ScarabResult, TileVec};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    window: Window,
    gamestate: Arc<RwLock<Gamestate<f64>>>,
    camera: Camera,
}

impl App {
    pub fn new(
        opengl: OpenGL,
        gamestate: Arc<RwLock<Gamestate<f64>>>,
        camera: Camera,
    ) -> ScarabResult<Self> {
        let window: Window = WindowSettings::new("spinning-square", [300, 400])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap(); // TODO: don't panic here

        Ok(Self {
            gl: GlGraphics::new(opengl),
            window,
            gamestate,
            camera,
        })
    }

    pub fn run(mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            // TODO: this is just a quick and easy POC
            // need to convert to controller based logic
            if let Some(args) = e.close_args() {
                self.close(&args);
                break;
            }

            if let Some(args) = e.render_args() {
                self.render(&args);
            }

            if let Some(args) = e.update_args() {
                self.update(&args);
            }

            match e {
                Event::Input(input, _i) => {
                    let mut gamestate = self.gamestate.write().unwrap();
                    gamestate.input_event(input);
                }
                _ => {}
            }
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |ctx, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let gamestate = self.gamestate.read().unwrap();
            gamestate.render(&self.camera, ctx, gl).unwrap();
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let mut gamestate = self.gamestate.write().unwrap();
        gamestate.update(args.dt);
    }

    fn close(&mut self, _args: &CloseArgs) {}
}
