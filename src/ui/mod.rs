pub mod categories;
pub mod tasks;
pub mod gauge;
pub mod widgets;
pub mod help;
pub mod milestones;
pub mod input;
pub mod timers;
pub mod user;


use ratatui::{layout::{Constraint, Direction, Layout, Rect}, symbols::border, widgets::{Block, Clear, Padding, Paragraph}, Frame};

use crate::app::App;



pub fn ui(frame: &mut Frame, app: &mut App){
    app.render_widgets(frame);
}

pub fn render_result(app: &mut App, frame: &mut Frame, width: u16, height: u16, area: Rect){
    let block = Block::bordered()
        .border_set(border::ROUNDED)
        .padding(Padding::new(3, 3, 1, 1));

    let paragraph = Paragraph::new(app.result_message.clone())
        .centered()
        .block(block)
        .style(app.theme.floating);

    let area = centered_rect(width, height, area);
    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}

pub fn render_error(app: &mut App, frame: &mut Frame, width: u16, height: u16, area: Rect){
    let block = Block::bordered()
        .border_set(border::ROUNDED)
        .padding(Padding::new(3, 3, 1, 1));

    let paragraph = Paragraph::new(app.error_message.clone())
        .centered()
        .block(block)
        .style(app.theme.error);

    let area = centered_rect(width, height, area);
    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
