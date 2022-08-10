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
};

enum Mode {
    Insert,
    Note,
    Mark,
    Show,
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
    pub fn run(
        &mut self,
        mut terminal: Terminal<CrosstermBackend<Stdout>>,
        events: Events,
    ) -> Terminal<CrosstermBackend<Stdout>> {
        loop {
            terminal
                .draw(|frame| {
                    let terminal_rect = frame.size();

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

                    let sudoku_rect = Rect {
                        x: 0,
                        y: 0,
                        width: 35,
                        height: 71,
                    };

                    let board_widget = BoardWidget {};

                    frame.render_stateful_widget(board_widget, sudoku_rect, self.board.borrow_mut())
                })
                .unwrap();

            let event = events.next().unwrap();

            match event {
                Event::Input(key) => {
                    match key {
                        // movement using arrow keys or vim movement keys
                        Key::Up | Key::Char('k') => self.board.move_up(),
                        Key::Down | Key::Char('j') => self.board.move_down(),
                        Key::Left | Key::Char('h') => self.board.move_left(),
                        Key::Right | Key::Char('l') => self.board.move_right(),
                        Key::Ctrl('k') => {
                            self.board.move_up();
                            self.board.move_up();
                            self.board.move_up();
                        }
                        Key::Ctrl('j') => {
                            self.board.move_down();
                            self.board.move_down();
                            self.board.move_down();
                            ();
                        }
                        Key::Ctrl('h') => {
                            self.board.move_left();
                            self.board.move_left();
                            self.board.move_left();
                        }
                        Key::Ctrl('l') => {
                            self.board.move_right();
                            self.board.move_right();
                            self.board.move_right();
                        }
                        Key::Char('i') => self.mode = Mode::Insert,
                        Key::Char('n') => self.mode = Mode::Note,
                        Key::Char('m') => self.mode = Mode::Mark,

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
                                Key::Char('1') => self.board.hightlight(1),
                                Key::Char('2') => self.board.hightlight(2),
                                Key::Char('3') => self.board.hightlight(3),
                                Key::Char('4') => self.board.hightlight(4),
                                Key::Char('5') => self.board.hightlight(5),
                                Key::Char('6') => self.board.hightlight(6),
                                Key::Char('7') => self.board.hightlight(7),
                                Key::Char('8') => self.board.hightlight(8),
                                Key::Char('9') => self.board.hightlight(9),
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