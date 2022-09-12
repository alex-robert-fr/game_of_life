use std::{time::Duration, collections::HashMap};

use crossterm::event::{
    poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseButton,
    MouseEventKind,
};
use game_engine::{
    render::{Pencil, Window},
    spacial::Vector2D,
    Engine, State,
};

#[derive(PartialEq, Eq)]
enum Life {
    Live,
    Death,
}

struct GameOfLife {
    cells: HashMap<Vector2D, Life>,
}

impl GameOfLife {
    pub fn new() -> Self {
        GameOfLife { cells: HashMap::new() }
    }
}

fn main() {
    let mut engine = Engine::new(30);
    let mut game = GameOfLife::new();
    engine.run(|state: &mut State, window: &mut Window| {
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
                                // game.cells.in(Cell {
                                //     pos: Vector2D::new(event.column as i32, event.row as i32),
                                //     life: Life::Live,
                                // });
                                game.cells.insert(Vector2D::new(event.column as i32, event.row as i32), Life::Live);
                            }
                        }
                        _ => (),
                    }
                }
            }
            Err(_) => (),
        }
        // pencil.draw_text("Hello", Vector2D::new(2, 2));
        for (cell, state_cell) in &game.cells {
            if state_cell == &Life::Live {
                pencil.draw_item('#', *cell);
            }
        }
    });
}
