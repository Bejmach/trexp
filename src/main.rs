mod ui;
mod app;
mod json_types;
mod timer;
mod events;
mod theme;

use std::{error::Error, io};

use ratatui::{crossterm::{event::{self, Event}, execute, terminal::{disable_raw_mode, EnterAlternateScreen}}, prelude::{Backend, CrosstermBackend}, text::Text, Frame, Terminal};

use crate::{app::App, events::handle_events, json_types::Category, ui::ui};

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    run_app(&mut terminal, &mut app)?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), io::Error>{
    while !app.exit{
        terminal.draw(|f| ui(f, app))?;
        handle_events(app)?;
    }
    Ok(())
}
