use std::collections::HashMap;

use ratatui::{layout::Rect, style::Stylize, symbols::border, text::{Line, Span}, widgets::{Block, Clear, Padding, Paragraph}, Frame};

use crate::{app::App, theme::StyleData, traits::tr_widget::TrWidget, ui::{centered_rect, widgets::{ConstraintFit, WidgetData}}, wild_type::Variant};

pub struct HelpWidget{}

impl HelpWidget{
    pub fn new() -> Self{Self {}}
}

impl TrWidget for HelpWidget{
    fn render(&mut self, frame: &mut Frame, app: &App, layout_data: &HashMap<String, Vec<Rect>>, data: &WidgetData) {
        let area = match data.constraint_fit{
                ConstraintFit::Default => layout_data.get(&data.layout).expect("no layout with provided id").get(data.constraint).expect("no constraint with provided id"),
                ConstraintFit::Centered { percent_x, percent_y } => &centered_rect(percent_x, percent_y, *layout_data.get(&data.layout).expect("no layout with provided id").get(data.constraint).expect("no constraint with provided id"))
            };

        let mut span_vec: Vec<Span> = Vec::new();
        let mut line_vec: Vec<Line> = Vec::new();
        
        let key_style = app.theme.help_key;
        let command_style = app.theme.help_text;
        let max_width = area.width - 10;
        let mut counter: u16 = 0;



        let keybinds = if let Some(Variant::Str(keybind_str)) = app.additional_data.get("last_state"){
            app.app_config.keybinds.get(keybind_str).expect("No keybinds for state")
        }else{
            &HashMap::new()
        };

        for (key, help_data) in keybinds {
            counter += key.chars().count() as u16 + help_data.info.chars().count() as u16 + 4;
            if counter >= max_width {
                line_vec.push(Line::from(span_vec.clone()));
                span_vec.clear();
                counter = 0;
            }
            span_vec.append(&mut vec![
                Span::styled(key, key_style),
                Span::raw(" "),
                Span::styled(help_data.info.clone(), command_style),
                Span::raw("   ")
            ]);
        }

        if !span_vec.is_empty() {
            line_vec.push(Line::from(span_vec.clone()));
        }

        let style_data: &StyleData = if let Some(data) = data.styles.get(&app.state){
            app.app_config.styles.get(data).expect("No style with provided name")
        }else if let Some(data) = data.styles.get("_"){
            app.app_config.styles.get(data).expect("No default style provoded")
        }else{
            &StyleData::new()
        };

        let style = style_data.to_style();

        let block = Block::bordered()
            .title(Line::from(" Categories ".bold()))
            .border_set(border::PLAIN)
            .padding(Padding::new(2, 4, 1, 1))
            .style(style);

        let paragraph = Paragraph::new(line_vec).block(block).style(app.theme.passive);
        frame.render_widget(Clear, *area);
        frame.render_widget(paragraph, *area);
    }
}
