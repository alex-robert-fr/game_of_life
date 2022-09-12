use std::time::Duration;

use crossterm::event::{
    poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseButton,
    MouseEventKind,
};
use game_engine::{
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
            let mut pencil = Pencil::new(window);

            match poll(Duration::from_millis(0)) {
                Ok(val) => {
                    if val {
                        match read().unwrap() {
                            Event::Key(KeyEvent {
                                code: KeyCode::Char('q'),
                                modifiers: KeyModifiers::NONE,
                                kind: KeyEventKind::Press,
                                state: KeyEventState::NONE,
                            })
                            | Event::Key(KeyEvent {
                                code: KeyCode::Char('Q'),
                                modifiers: KeyModifiers::SHIFT,
                                kind: KeyEventKind::Press,
                                state: KeyEventState::NONE,
                            }) => {
                                state.exit = true;
                            }
                            Event::Mouse(event) => {
                                if event.kind == MouseEventKind::Down(MouseButton::Left) {
                                    pencil.draw_item('#', Vector2D::new(event.column as i32, event.row as i32));
                                }
                            }
                            _ => (),
                        }
                    }
                }
                Err(_) => (),
            }
            pencil.draw_text("Hello", Vector2D::new(2, 2));
        },
    );
}
