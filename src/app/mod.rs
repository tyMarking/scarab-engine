use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, ButtonArgs, ButtonEvent, ButtonState, CloseArgs, CloseEvent, Key};

use crate::{Camera, Gamestate, ScarabResult, TileVec};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    window: Window,
    gamestate: Gamestate<f64>,
    camera: Camera,
}

impl App {
    pub fn new(opengl: OpenGL, gamestate: Gamestate<f64>, camera: Camera) -> ScarabResult<Self> {
        let window: Window = WindowSettings::new("spinning-square", [200, 200])
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

            if let Some(args) = e.button_args() {
                self.button(&args);
            }
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |ctx, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            self.gamestate.render(&self.camera, ctx, gl).unwrap();
        });
    }

    fn update(&mut self, args: &UpdateArgs) {}

    fn close(&mut self, _args: &CloseArgs) {}

    fn button(&mut self, args: &ButtonArgs) {
        match args.button {
            Button::Keyboard(key) => match key {
                Key::W | Key::Up => {
                    // Up is -y
                    if args.state == ButtonState::Press {
                        self.player_move(TileVec::new(0.0, -1.0))
                    }
                }
                Key::A | Key::Left => {
                    if args.state == ButtonState::Press {
                        self.player_move(TileVec::new(-1.0, 0.0))
                    }
                }
                Key::S | Key::Down => {
                    // Down is +y
                    if args.state == ButtonState::Press {
                        self.player_move(TileVec::new(0.0, 1.0))
                    }
                }
                Key::D | Key::Right => {
                    if args.state == ButtonState::Press {
                        self.player_move(TileVec::new(1.0, 0.0))
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn player_move(&mut self, direction: TileVec<f64>) {
        self.gamestate.player_mut().map_or_else(
            || println!("no player!"),
            |p| {
                let _ = p.try_move(direction).or_else(|e| {
                    println!("Couldn't move: {:?}", e);
                    Err(e)
                });
            },
        )
    }
}
