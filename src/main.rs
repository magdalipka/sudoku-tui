mod board;
mod cell;
mod cell_options;
mod column;
mod events;
mod grid;
mod row;
mod ui;

use crate::events::Events;
use core::time;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread};
use tui::{backend::CrosstermBackend, Terminal};
use ui::UI;

fn main() -> Result<(), io::Error> {
    // let grid: Grid = Grid::default();

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut ui = UI::default();

    let mut terminal = ui.run(terminal, events);

    thread::sleep(time::Duration::from_millis(2000));

    // restore terminal
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    disable_raw_mode()?;

    Ok(())
}
