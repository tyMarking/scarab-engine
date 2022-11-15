use opengl_graphics::OpenGL;

use piston::{Button, Key};
use scarab_engine::{
    control::UpdateChannel,
    gameobject::{
        entity::EntityControls,
        field::{Cell, Field},
        Entity, AIR, SOLID,
    },
    playercontroller::{Axis2d, InputController, InputHandler},
    App, Camera, Gamestate, HasBoxMut, PhysBox, ScarabResult, TileVec,
};

fn main() -> ScarabResult<()> {
    let cell = Cell::new(SOLID, [0.0, 0.0, 0.0, 1.0], PhysBox::new((0, 0), (10, 20))?);
    let cell2 = Cell::new(
        SOLID,
        [0.0, 0.0, 0.0, 1.0],
        PhysBox::new((10, 10), (20, 30))?,
    );
    let cell3 = Cell::new(AIR, [1.0; 4], PhysBox::new((10, 0), (40, 10))?);
    let cell4 = Cell::new(AIR, [1.0; 4], PhysBox::new((30, 10), (20, 50))?);
    let cell5 = Cell::new(AIR, [1.0; 4], PhysBox::new((0, 40), (30, 20))?);
    let cell6 = Cell::new(AIR, [1.0; 4], PhysBox::new((0, 20), (10, 20))?);
    let cell7 = Cell::new(SOLID, [0.0, 0.0, 0.0, 1.0], PhysBox::new((50, 0), (1, 60))?);
    let cell8 = Cell::new(SOLID, [0.0, 0.0, 0.0, 1.0], PhysBox::new((0, 60), (50, 1))?);

    let field = Field::new(vec![cell, cell2, cell3, cell4, cell5, cell6, cell7, cell8])?;

    let mut gamestate = Gamestate::new(field);

    let cambox = PhysBox::new((0, 0), (100, 100))?;
    let camera = Camera::new(5, cambox, TileVec::new(1000, 1000));

    let mut p = Entity::new_def()?;
    let b = p.get_box_mut();
    b.set_pos(TileVec::new(31.0, 21.0))?;
    b.set_size(TileVec::new(8.0, 8.0))?;
    p.set_max_velocity(8.0);

    let sender = p.get_sender();
    let mut controller = InputController::new(sender);
    controller
        .bind_axis(
            Button::Keyboard(Key::D),
            Button::Keyboard(Key::A),
            Button::Keyboard(Key::S),
            Button::Keyboard(Key::W),
            InputHandler::new(Axis2d::default(), |a: Axis2d| {
                EntityControls::SetMovement(a.into())
            }),
        )
        .unwrap();

    gamestate.add_entity(p);
    gamestate.add_input_controller(controller);

    let app = App::new(OpenGL::V3_2, gamestate, camera).unwrap();
    app.run();
    Ok(())
}
