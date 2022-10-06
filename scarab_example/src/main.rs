use scarab_engine::{app::App, OpenGL};

fn main() {
    let app = App::new(OpenGL::V3_2).unwrap();
    app.run();
}
