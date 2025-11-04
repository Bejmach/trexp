mod app;
mod events;
mod ui;
mod traits;
mod layout_conf;
mod timer;
mod json_types;
mod theme;
mod wild_type;

use std::{error::Error, fs::File, io::{self, Read}, time::{Duration, Instant}};

use ratatui::{prelude::Backend, Terminal};

use crate::{app::App, events::handle_events, json_types::Category, ui::ui};

fn main() -> Result<(), Box<dyn Error>> {

    let mut file = File::open("config.json")?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;

    let mut terminal = ratatui::init();
    let mut app = App::new();

    app.load_config(serde_json::from_str(&data).expect("couldnt parse data"));
    app.init();

    run_app(&mut terminal, &mut app)?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), io::Error>{
    let tick_rate = Duration::from_secs_f64(0.25);
    let mut last_tick = Instant::now();

    while !app.exit{
        terminal.draw(|f| ui(f, app))?;

        app.handle_timers();

        let timeout: Duration = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        handle_events(app, timeout)?;

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
    Ok(())
}
