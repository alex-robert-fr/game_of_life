use game_engine::{
    input::Key,
    render::{Pencil, Window},
    spacial::Vector2D,
    Engine, Game, State,
};

struct Cell {
    pos: Vector2D,
    life: Life,
}

enum Life {
    Live,
    Death,
}

struct GameOfLife {
    cells: Vec<Cell>,
}

impl GameOfLife {
    pub fn new() -> Self {
        GameOfLife { cells: Vec::new() }
    }
}

fn main() {
    let mut engine = Engine::new(30);
    engine.run(
        |mut game: &mut Game, state: &mut State, window: &mut Window| {
            for key in state.keyboard.get_last_key_press() {
                match key {
                    Key::Q => state.exit = true,
                    Key::Unknown => break,
                }
            }

            let mut pencil = Pencil::new(window);
            pencil.draw_text("Hello", Vector2D::new(10, 2));
        },
    );
}
