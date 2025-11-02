use std::collections::HashMap;

use ratatui::{layout::Rect, text::{Line, Span}, widgets::Paragraph, Frame};

use crate::{app::App, traits::tr_widget::TrWidget, ui::{centered_rect, widgets::{ConstraintFit, WidgetData}}};

pub struct HelpWidget{}

impl HelpWidget{
    pub fn new() -> Self{Self {}}
}

impl TrWidget for HelpWidget{
    fn render(&self, frame: &mut Frame, app: &App, layout_data: &HashMap<String, Vec<Rect>>, data: &WidgetData) {
        let area = match data.constraint_fit{
                ConstraintFit::Default => layout_data.get(&data.layout).expect("no layout with provided id").get(data.constraint).expect("no constraint with provided id"),
                ConstraintFit::Center { percent_x, percent_y } => &centered_rect(percent_x, percent_y, *layout_data.get(&data.layout).expect("no layout with provided id").get(data.constraint).expect("no constraint with provided id"))
            };

        let mut span_vec: Vec<Span> = Vec::new();
        let mut line_vec: Vec<Line> = Vec::new();
        
        let key_style = app.theme.help_key;
        let command_style = app.theme.help_text;
        let max_width = frame.area().width - 10;
        let mut counter: u16 = 0;

        let keybinds = app.app_config.keybinds.get(&app.state).expect("No keybinds for state");

        for (key, command) in keybinds {
            counter += key.chars().count() as u16 + command.chars().count() as u16 + 4;
            if counter >= max_width {
                line_vec.push(Line::from(span_vec.clone()));
                span_vec.clear();
                counter = 0;
            }
            span_vec.append(&mut vec![
                Span::styled(key, key_style),
                Span::raw(" "),
                Span::styled(command, command_style),
                Span::raw("   ")
            ]);
        }

        if !span_vec.is_empty() {
            line_vec.push(Line::from(span_vec.clone()));
        }

        let paragraph = Paragraph::new(line_vec).style(app.theme.passive);
        frame.render_widget(paragraph, *area);
    }
}
