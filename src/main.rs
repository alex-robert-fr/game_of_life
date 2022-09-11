use game_engine::{Engine, Game, State, input::Key};

struct GameOfLife {

}

impl GameOfLife {
    pub fn new() -> Self {

        GameOfLife {  }
    }
}

fn main() {
    let mut engine = Engine::new(30);
    engine.run(|mut game: &mut Game, state: &mut State| {
        for key in state.keyboard.get_last_key_press() {
            match key {
                Key::Q => state.exit = true,
                Key::Unknown => ()
            }
        }
        println!("Run Game");
    });
}
