mod app;
mod events;
mod ui;
mod traits;
mod layout_conf;
mod timer;
mod json_types;
mod theme;
mod wild_type;

use std::{error::Error, fs::File, io::{self, Read}, path::PathBuf, time::{Duration, Instant}};

use dirs_next::config_dir;
use ratatui::{prelude::Backend, Terminal};

use crate::{app::App, events::handle_events, json_types::Category, ui::ui};

fn get_config_dir(app_name: &str) -> Option<PathBuf> {
    dirs_next::config_dir().map(|base| base.join(app_name))
}

fn main() -> Result<(), Box<dyn Error>> {
    if let Some(config_path) = get_config_dir("trexp"){
        let mut file = File::open(config_path.join("config.json").to_str().expect("no config path provided").to_string())?;
        let mut data = String::new();

        file.read_to_string(&mut data)?;

        let mut terminal = ratatui::init();
        let mut app = App::new();

        app.load_config(serde_json::from_str(&data).expect("couldnt parse data"));
        app.init(config_path);

        run_app(&mut terminal, &mut app)?;

        Ok(())
    }
    else{
        println!("Couldn't find config path");
        Ok(())
    }
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
