use std::{io, time::Duration};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::{App};

pub fn handle_events(app: &mut App, timeout: Duration) -> io::Result<()>{
    if event::poll(timeout)?{
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                handle_key_event(app, key_event);
            },
            _ => {}
        };
    }
    Ok(())
}

fn handle_key_event(app: &mut App, key_event: KeyEvent){
    if app.error_message != String::new() || app.result_message != String::new() {
        match key_event.code {
            KeyCode::Char('q') => {
                app.error_message = String::new();
                app.result_message = String::new();
            },
            _ => {}
        }
    }
    else if let Some(event_config) = app.app_config.keybinds.get(&app.state){
        let key_str = key_event_to_string(key_event);
        if let Some(command) = event_config.get(&key_str){
            app.run_command_string(command.to_string());
        }
    }
}

fn key_event_to_string(event: KeyEvent) -> String {
    let mut parts = vec![];

    if event.modifiers.contains(KeyModifiers::CONTROL) {
        parts.push("Ctrl");
    }
    if event.modifiers.contains(KeyModifiers::ALT) {
        parts.push("Alt");
    }
    if event.modifiers.contains(KeyModifiers::SHIFT) {
        parts.push("Shift");
    }

    let key_str = match event.code {
        KeyCode::Char(c) => format!("C({})", c),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Esc => "Esc".to_string(),
        KeyCode::Backspace => "Backspace".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::Left => "Left".to_string(),
        KeyCode::Right => "Right".to_string(),
        KeyCode::Up => "Up".to_string(),
        KeyCode::Down => "Down".to_string(),
        KeyCode::Home => "Home".to_string(),
        KeyCode::End => "End".to_string(),
        KeyCode::PageUp => "PageUp".to_string(),
        KeyCode::PageDown => "PageDown".to_string(),
        KeyCode::Delete => "Delete".to_string(),
        KeyCode::Insert => "Insert".to_string(),
        _ => return String::from("<unhandled>"),
    };

    parts.push(&key_str);

    parts.join("+")
}
