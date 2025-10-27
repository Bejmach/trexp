use std::io;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::app::{App, AppComponent, AppEdit, AppState};

pub fn handle_events(app: &mut App) -> io::Result<()>{
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            handle_key_event(app, key_event);
        },
        _ => {}
    };
    Ok(())
}

fn handle_key_event(app: &mut App, key_event: KeyEvent){
    if app.error_message != "" {
        match key_event.code{
            KeyCode::Enter => app.error_message = String::new(),
            _ => {},
        }
    }
    else{
        match app.state{
            AppState::Main => main_key_events(app, key_event),
            AppState::Categories => categories_key_events(app, key_event),
            AppState::CreateCategory => create_category_key_events(app, key_event),
            AppState::Tasks => tasks_key_events(app, key_event),
            AppState::CreateTask => create_task_key_events(app, key_event),
            AppState::Milestones => milestones_key_events(app, key_event),
            AppState::CreateMilestone => create_milestone_key_events(app, key_event),
            AppState::Timers => timers_key_events(app, key_event),
            _ => {}
        }
    }
}

fn main_key_events(app: &mut App, key_event: KeyEvent){
    match key_event.code{
        KeyCode::Char('q') => app.exit = true,
        KeyCode::Up => app.move_cursor_up(),
        KeyCode::Down => app.move_cursor_down(),
        KeyCode::Left => app.move_cursor_left(),
        KeyCode::Right => app.move_cursor_right(),
        KeyCode::Enter => {
            if let Some(&component) = app.get_cur_component(){
                match component{
                    AppComponent::Categories => app.set_state(AppState::Categories),
                    AppComponent::Tasks => app.set_state(AppState::Tasks),
                    AppComponent::Milestones => app.set_state(AppState::Milestones),
                    AppComponent::Timers => app.set_state(AppState::Timers),
                }
            }
        },
        _ => {},
    }
}

fn categories_key_events(app: &mut App, key_event: KeyEvent){
    match key_event.code{
        KeyCode::Char('q') => app.state = AppState::Main,
        KeyCode::Char('n') => app.state = AppState::CreateCategory,
        KeyCode::Char('T') => {/*Create timer*/},
        KeyCode::Up => app.id_up(),
        KeyCode::Down => app.id_down(),
        KeyCode::PageUp => {
            if app.data.move_category(app.cur_category as usize, -1).is_ok(){
                app.cur_category -= 1;
            }
        },
        KeyCode::PageDown => {
            if app.data.move_category(app.cur_category as usize, 1).is_ok(){
                app.cur_category += 1;
            }
        }
        _ => {}
    }
}

fn create_category_key_events(app: &mut App, key_event: KeyEvent){
    match key_event.code{
        KeyCode::Esc => {
            app.set_state(AppState::Categories);
            app.edit_name = String::new();
        },
        KeyCode::Enter => {
            if app.save_category().is_err(){
                app.error_message = "Error while creating category".to_string();
            }
            app.set_state(AppState::Categories);
        },
        KeyCode::Backspace => {
            app.edit_name.pop();
        }
        KeyCode::Char(value) => {
            app.edit_name.push(value);
        },
        _ => {},
    }
}

fn tasks_key_events(app: &mut App, key_event: KeyEvent){
    match key_event.code{
        KeyCode::Char('q') => app.state = AppState::Main,
        KeyCode::Char('n') => {
            app.state = AppState::CreateTask;
            app.cur_edit = AppEdit::Name;
        },
        KeyCode::Up => app.id_up(),
        KeyCode::Down => app.id_down(),
        KeyCode::PageUp => {
            if let Some(category) = app.data.get_category_mut(app.cur_category as usize){
                if category.move_task(app.cur_task as usize, -1).is_ok(){
                    app.cur_task -=1;
                }
            }
        },
        KeyCode::PageDown => {
            if let Some(category) = app.data.get_category_mut(app.cur_category as usize){
                if category.move_task(app.cur_task as usize, 1).is_ok(){
                    app.cur_task +=1;
                }
            }
        }
        _ => {},
    }
}

fn create_task_key_events(app: &mut App, key_event: KeyEvent){
    match key_event.code {
        KeyCode::Enter => {
            if app.save_task().is_err(){
                app.error_message = "Error while creating task".to_string();
            }
            app.set_state(AppState::Tasks);
            app.edit_name = String::new();
            app.edit_exp = String::new();
            app.cur_edit = AppEdit::None;
        },
        KeyCode::Esc => {
            app.set_state(AppState::Tasks);
            app.edit_name = String::new();
            app.edit_exp = String::new();
        },
        KeyCode::Backspace => {
            if app.cur_edit == AppEdit::Name{
                app.edit_name.pop();
            }
            else if app.cur_edit == AppEdit::Exp{
                app.edit_exp.pop();
            }
        }
        KeyCode::Right => {
            app.cur_edit = AppEdit::Exp;
        }
        KeyCode::Left => {
            app.cur_edit = AppEdit::Name;
        }
        KeyCode::Char(value) => {
            match value{
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                    if app.cur_edit == AppEdit::Exp{
                        app.edit_exp.push(value);
                    }
                    else if app.cur_edit == AppEdit::Name{
                        app.edit_name.push(value);
                    }
                }
                _ => {
                    if app.cur_edit == AppEdit::Name{
                        app.edit_name.push(value);
                    }
                }
            }
        },
        _ => {}
    }
}

fn milestones_key_events(app: &mut App, key_event: KeyEvent){
    match key_event.code{
        KeyCode::Char('q') => app.state = AppState::Main,
        KeyCode::Char('n') => {
            app.state = AppState::CreateMilestone;
            app.cur_edit = AppEdit::Name;
        },
        KeyCode::Up => app.id_up(),
        KeyCode::Down => app.id_down(),
        KeyCode::PageUp => {
            if let Some(category) = app.data.get_category_mut(app.cur_category as usize){
                if category.move_milestone(app.cur_milestone as usize, -1).is_ok(){
                    app.cur_milestone -=1;
                }
            }
        },
        KeyCode::PageDown => {
            if let Some(category) = app.data.get_category_mut(app.cur_category as usize){
                if category.move_milestone(app.cur_milestone as usize, 1).is_ok(){
                    app.cur_milestone +=1;
                }
            }
        }
        _ => {},
    }
}

fn create_milestone_key_events(app: &mut App, key_event: KeyEvent){
    match key_event.code {
        KeyCode::Enter => {
            if app.save_milestone().is_err(){
                app.error_message = "Error while creating milestone".to_string();
            }
            app.set_state(AppState::Milestones);
            app.edit_name = String::new();
            app.edit_exp = String::new();
            app.cur_edit = AppEdit::None;
        },
        KeyCode::Esc => {
            app.set_state(AppState::Milestones);
            app.edit_name = String::new();
            app.edit_exp = String::new();
        },
        KeyCode::Backspace => {
            if app.cur_edit == AppEdit::Name{
                app.edit_name.pop();
            }
            else if app.cur_edit == AppEdit::Exp{
                app.edit_exp.pop();
            }
        }
        KeyCode::Right => {
            app.cur_edit = AppEdit::Exp;
        }
        KeyCode::Left => {
            app.cur_edit = AppEdit::Name;
        }
        KeyCode::Char(value) => {
            match value{
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                    if app.cur_edit == AppEdit::Exp{
                        app.edit_exp.push(value);
                    }
                    else if app.cur_edit == AppEdit::Name{
                        app.edit_name.push(value);
                    }
                }
                _ => {
                    if app.cur_edit == AppEdit::Name{
                        app.edit_name.push(value);
                    }
                }
            }
        },
        _ => {}
    }
}

fn timers_key_events(app: &mut App, key_event: KeyEvent){
    match key_event.code{
        KeyCode::Char('q') => app.state = AppState::Main,
        _ => {},
    }
}
