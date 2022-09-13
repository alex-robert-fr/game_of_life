use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    event::{
        poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers,
        MouseButton, MouseEventKind,
    },
    terminal::size,
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
    cells: Vec<Vec<Life>>,
}

impl GameOfLife {
    pub fn new() -> Self {
        let size = size().unwrap();
        let mut cells = Vec::new();
        for _ in 0..size.1 {
            let mut x_line = Vec::new();
            for _ in 0..size.0 {
                x_line.push(Life::Death);
            }
            cells.push(x_line);
        }
        GameOfLife { cells }
    }

    pub fn update(&mut self) {
        let cells = &mut self.cells;
        for y in 0..cells.len() {
            for x in 0..cells[y].len() {
                if cells[y][x] == Life::Live {
                    if cells[y][x - 1] == Life::Live {
                        cells[y][x] = Life::Death;
                    }
                }
            }
        }
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
                                // game.cells.insert(
                                //     Vector2D::new(event.column as i32, event.row as i32),
                                //     Life::Live,
                                // );
                                game.cells[event.row as usize][event.column as usize] = Life::Live;
                            }
                        }
                        _ => (),
                    }
                }
            }
            Err(_) => (),
        }

        game.update();

        // pencil.draw_text("Hello", Vector2D::new(2, 2));
        for (y, cell) in game.cells.iter().enumerate() {
            for (x, life) in cell.iter().enumerate() {
                if life == &Life::Live {
                    pencil.draw_item(
                        '#',
                        Vector2D {
                            x: x as i32,
                            y: y as i32,
                        },
                    );
                } else {
                    pencil.draw_item(
                        ' ',
                        Vector2D {
                            x: x as i32,
                            y: y as i32,
                        },
                    )
                }
            }
        }
    });
}
