use std::{borrow::BorrowMut, io::Stdout};

use crossterm::event::KeyModifiers;
use termion::event::Key;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};

use crate::{
    board::{Board, BoardWidget},
    events::{Event, Events},
    grid::Grid,
};

#[derive(PartialEq)]
enum Mode {
    Insert,
    Note,
    Mark,
    Show,
    Features,
}

pub struct UI {
    board: Board,
    mode: Mode,
}

impl Default for UI {
    fn default() -> Self {
        Self {
            board: Board::default(),
            mode: Mode::Insert,
        }
    }
}

impl UI {
    pub fn from(grid: Grid) -> Self {
        Self {
            board: Board::from(grid),
            mode: Mode::Insert,
        }
    }

    pub fn run(
        &mut self,
        mut terminal: Terminal<CrosstermBackend<Stdout>>,
        events: Events,
    ) -> Terminal<CrosstermBackend<Stdout>> {
        loop {
            terminal
                .draw(|frame| {
                    let terminal_rect = frame.size();

                    // println!("{}, {}", terminal_rect.height, terminal_rect.width);

                    if terminal_rect.width < 80 || terminal_rect.height < 40 {
                        let message = Paragraph::new("Window is too small\nPlease expand window")
                            .alignment(Alignment::Center);
                        frame.render_widget(message, terminal_rect);

                        return ();
                    }

                    let outer_block = Block::default()
                        .borders(Borders::ALL)
                        .title(Span::styled(
                            "sudoku",
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD),
                        ))
                        .border_type(BorderType::Rounded);
                    frame.render_widget(outer_block, terminal_rect);

                    if self.mode == Mode::Features {
                        let menu = Paragraph::new(
                            "Press button to select action:\n 1. Auto-fill \n 2. Hint",
                        )
                        .block(Block::default().title("Paragraph").borders(Borders::ALL));

                        frame.render_widget(menu, terminal_rect);
                    } else {
                        let board_widget = BoardWidget {};
                        frame.render_stateful_widget(
                            board_widget,
                            terminal_rect,
                            self.board.borrow_mut(),
                        )
                    }

                    // let menu_widget = Block::default();

                    // frame.render_widget(widget, area)
                })
                .unwrap();

            let event = events.next().unwrap();

            match event {
                Event::Input(key) => {
                    match key {
                        // movement using arrow keys or vim movement keys
                        Key::Up | Key::Char('w') => self.board.move_up(),
                        Key::Down | Key::Char('s') => self.board.move_down(),
                        Key::Left | Key::Char('a') => self.board.move_left(),
                        Key::Right | Key::Char('d') => self.board.move_right(),
                        Key::Char('W') => {
                            self.board.move_up();
                            self.board.move_up();
                            self.board.move_up();
                        }
                        Key::Char('S') => {
                            self.board.move_down();
                            self.board.move_down();
                            self.board.move_down();
                        }
                        Key::Char('A') => {
                            self.board.move_left();
                            self.board.move_left();
                            self.board.move_left();
                        }
                        Key::Char('D') => {
                            self.board.move_right();
                            self.board.move_right();
                            self.board.move_right();
                        }
                        Key::Char('i') => self.mode = Mode::Insert,
                        Key::Char('n') => self.mode = Mode::Note,
                        Key::Char('m') => self.mode = Mode::Mark,
                        Key::Char('h') => self.mode = Mode::Show,
                        Key::Char('f') => {
                            if self.mode == Mode::Features {
                                self.mode = Mode::Insert;
                            } else {
                                self.mode = Mode::Features;
                            }
                        }
                        Key::Char('c') => self.board.reset_colors(),

                        Key::Ctrl('c') => break,
                        _ => match self.mode {
                            Mode::Insert => match key {
                                Key::Char('1') => self.board.set_value(1),
                                Key::Char('2') => self.board.set_value(2),
                                Key::Char('3') => self.board.set_value(3),
                                Key::Char('4') => self.board.set_value(4),
                                Key::Char('5') => self.board.set_value(5),
                                Key::Char('6') => self.board.set_value(6),
                                Key::Char('7') => self.board.set_value(7),
                                Key::Char('8') => self.board.set_value(8),
                                Key::Char('9') => self.board.set_value(9),
                                Key::Char('0') => self.board.set_value(0),
                                _ => {}
                            },
                            Mode::Note => match key {
                                Key::Char('1') => self.board.toggle_option(1),
                                Key::Char('2') => self.board.toggle_option(2),
                                Key::Char('3') => self.board.toggle_option(3),
                                Key::Char('4') => self.board.toggle_option(4),
                                Key::Char('5') => self.board.toggle_option(5),
                                Key::Char('6') => self.board.toggle_option(6),
                                Key::Char('7') => self.board.toggle_option(7),
                                Key::Char('8') => self.board.toggle_option(8),
                                Key::Char('9') => self.board.toggle_option(9),
                                _ => {}
                            },
                            Mode::Show => match key {
                                Key::Char('1') => self.board.highlight(1),
                                Key::Char('2') => self.board.highlight(2),
                                Key::Char('3') => self.board.highlight(3),
                                Key::Char('4') => self.board.highlight(4),
                                Key::Char('5') => self.board.highlight(5),
                                Key::Char('6') => self.board.highlight(6),
                                Key::Char('7') => self.board.highlight(7),
                                Key::Char('8') => self.board.highlight(8),
                                Key::Char('9') => self.board.highlight(9),
                                _ => {}
                            },
                            Mode::Mark => match key {
                                Key::Char('1') => self.board.mark(1),
                                Key::Char('2') => self.board.mark(2),
                                Key::Char('3') => self.board.mark(3),
                                Key::Char('4') => self.board.mark(4),
                                Key::Char('5') => self.board.mark(5),
                                Key::Char('6') => self.board.mark(6),
                                Key::Char('7') => self.board.mark(7),
                                Key::Char('8') => self.board.mark(8),
                                Key::Char('9') => self.board.mark(9),
                                _ => {}
                            },
                            Mode::Features => match key {
                                Key::Esc => self.mode = Mode::Insert,
                                Key::Char('1') => {
                                    self.board.autofill();
                                    self.mode = Mode::Insert
                                }
                                _ => {}
                            },

                            _ => {}
                        },
                    }
                }
                Event::Tick => (),
            }
        }
        terminal
    }
}
