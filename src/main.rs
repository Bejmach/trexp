mod ui;
mod app;
mod json_types;
mod timer;
mod events;
mod theme;

use std::{error::Error, io, time::{Duration, Instant}};

use ratatui::{crossterm::{event::{self, Event}, execute, terminal::{disable_raw_mode, EnterAlternateScreen}}, prelude::{Backend, CrosstermBackend}, text::Text, Frame, Terminal};

use crate::{app::App, events::handle_events, json_types::Category, ui::ui};

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    let _ = app.load_data();

    run_app(&mut terminal, &mut app)?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), io::Error>{
    let tick_rate = Duration::from_secs(1);
    let mut last_tick = Instant::now();

    while !app.exit{
        terminal.draw(|f| ui(f, app))?;

        let timeout: Duration = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        handle_events(app, timeout)?;

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
    let _ = app.save_data();
    Ok(())
}
