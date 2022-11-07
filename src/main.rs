use opengl_graphics::OpenGL;

use scarab_engine::{
    gameobject::{
        field::{Cell, Field},
        Entity, AIR, SOLID,
    },
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

    gamestate.add_entity(p);

    let app = App::new(OpenGL::V3_2, gamestate, camera).unwrap();
    app.run();
    Ok(())
}
