use std::{
    io::{stdout, Write},
    thread,
    time::Duration, ops::Not,
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

#[derive(PartialEq, Eq, Clone, Copy)]
enum Life {
    Live,
    Death,
}

impl Not for Life {
    type Output = Life;

    fn not(self) -> Self {
        if self == Life::Live {
            Life::Death
        } else {
            Life::Live
        }
    }
}

struct GameOfLife {
    cells: Vec<Vec<Life>>,
    is_run: bool,
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
        GameOfLife {
            cells,
            is_run: false,
        }
    }

    pub fn update(&mut self) {

        let mut death: Vec<Vector2D> = Vec::new();
        let mut live: Vec<Vector2D> = Vec::new();

        let cells = &mut self.cells;
        for y in 0..cells.len() {
            for x in 0..cells[y].len() {
                let num_life_cells = GameOfLife::see(cells.clone(), y, x);
                if num_life_cells == 3 || (cells[y][x] == Life::Live && num_life_cells == 2) {
                    live.push(Vector2D {
                        x: x as i32,
                        y: y as i32,
                    });
                } else {
                    death.push(Vector2D {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
        for pos in live.iter() {
            cells[pos.y as usize][pos.x as usize] = Life::Live;
        }
        for pos in death.iter() {
            cells[pos.y as usize][pos.x as usize] = Life::Death;
        }
        thread::sleep(Duration::from_millis(1_000 / 30));
    }

    pub fn see(cells: Vec<Vec<Life>>, y: usize, x: usize) -> i32 {
        let mut counter = 0;
        let x_minus = if x == 0 {
            size().unwrap().0 as usize - 1
        } else {
            x - 1
        };
        let x_plus = if x == size().unwrap().0 as usize - 1 {
            0
        } else {
            x + 1
        };
        let y_minus = if y == 0 {
            size().unwrap().1 as usize - 1
        } else {
            y - 1
        };
        let y_plus = if y == size().unwrap().1 as usize - 1 {
            0
        } else {
            y + 1
        };
        // Left
        if cells[y][x_minus] == Life::Live {
            counter += 1;
        }
        // Left Up
        if cells[y_minus][x_minus] == Life::Live {
            counter += 1;
        }
        // Up
        if cells[y_minus][x] == Life::Live {
            counter += 1;
        }
        // Right Up
        if cells[y_minus][x_plus] == Life::Live {
            counter += 1;
        }
        // Right
        if cells[y][x_plus] == Life::Live {
            counter += 1;
        }
        // Right Down
        if cells[y_plus][x_plus] == Life::Live {
            counter += 1;
        }
        // Down
        if cells[y_plus][x] == Life::Live {
            counter += 1;
        }
        // Left Down
        if cells[y_plus][x_minus] == Life::Live {
            counter += 1;
        }
        counter
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
                        Event::Key(KeyEvent {
                            code: KeyCode::Char(' '),
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            state: KeyEventState::NONE,
                        }) => {
                            game.is_run = !game.is_run;
                        }
                        Event::Mouse(event) => {
                            if event.kind == MouseEventKind::Down(MouseButton::Left) {
                                game.cells[event.row as usize][event.column as usize] = !game.cells[event.row as usize][event.column as usize];
                            }
                        }
                        _ => (),
                    }
                }
            }
            Err(_) => (),
        }

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

        if game.is_run {
            pencil.draw_text("RUN", Vector2D { x: 0, y: 0 });
            game.update();
        } else {
            pencil.draw_text("PAUSE", Vector2D { x: 0, y: 0 });
        }
    });
}
