use opengl_graphics::OpenGL;

use scarab_engine::{
    gameobject::{
        field::{Cell, Field},
        Entity, SOLID,
    },
    App, Camera, Gamestate, HasBoxMut, PhysBox, TileVec,
};

fn main() {
    let cell = Cell {
        i: 0,
        solidity: SOLID,
        color: [1.0; 4],
        physbox: PhysBox::from_pos_size(TileVec::new(0, 0), TileVec::new(10, 10)),
    };
    let cell2 = Cell {
        i: 1,
        solidity: SOLID,
        color: [0.0, 0.0, 0.0, 1.0],
        physbox: PhysBox::from_pos_size(TileVec::new(10, 10), TileVec::new(20, 30)),
    };
    let cell3 = Cell {
        i: 2,
        solidity: SOLID,
        color: [0.5, 0.5, 0.5, 1.0],
        physbox: PhysBox::from_pos_size(TileVec::new(0, 10), TileVec::new(10, 10)),
    };
    let field = Field::new(vec![cell, cell2, cell3]);

    let mut gamestate = Gamestate::new(field);
    let cambox = PhysBox::from_pos_size(TileVec::new(0, 0), TileVec::new(50, 50));
    let camera = Camera::new(10, cambox, TileVec::new(1000, 1000));

    let mut p = Entity::default();
    let b = p.get_box_mut();
    b.set_pos(TileVec::new(11.0, 11.0));
    b.set_size(TileVec::new(8.0, 8.0));

    gamestate.add_entity(p);

    let app = App::new(OpenGL::V3_2, gamestate, camera).unwrap();
    app.run();
}
