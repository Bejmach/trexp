use std::collections::HashMap;

use ratatui::{layout::Rect, style::Stylize, symbols::border, text::Line, widgets::{Block, Clear, Padding, Paragraph}, Frame};

use crate::{app::App, theme::StyleData, traits::tr_widget::TrWidget, ui::{centered_rect, widgets::{ConstraintFit, WidgetData}}, wild_type::Variant};

pub struct InputWidget{
    pub value_name: String,
}

impl InputWidget{
    pub fn new(value_name: String) -> Self{
        Self { value_name }
    }
}

impl TrWidget for InputWidget{
    fn render(&self, frame: &mut Frame, app: &App, layout_data: &HashMap<String, Vec<Rect>>, data: &WidgetData) {
        let area = match data.constraint_fit{
                ConstraintFit::Default => layout_data.get(&data.layout).expect("no layout with provided id").get(data.constraint).expect("no constraint with provided id"),
                ConstraintFit::Centered { percent_x, percent_y } => &centered_rect(percent_x, percent_y, *layout_data.get(&data.layout).expect("no layout with provided id").get(data.constraint).expect("no constraint with provided id"))
            };        

        let text_value: String = if Some(self.value_name.clone()) == app.buffer_name{
            app.input_buffer.clone()
        }else if let Some(Variant::Str(value)) = app.additional_data.get(&self.value_name){
            value.to_string()
        }else if let Some(Variant::Int(value)) = app.additional_data.get(&self.value_name){
            value.to_string()
        }else{
            String::new()
        };

        let style_data: &StyleData = if let Some(data) = data.styles.get(&app.state){
            app.app_config.styles.get(data).expect("No style with provided name")
        }else if let Some(data) = data.styles.get("_"){
            app.app_config.styles.get(data).expect("No default style provoded")
        }else{
            &StyleData::new()
        };

        let style = style_data.to_style();

        let block = Block::bordered()
            .title(Line::from(self.value_name.clone().bold()))
            .border_set(border::PLAIN)
            .padding(Padding::new(2, 4, 1, 1))
            .style(style);

        let paragraph = Paragraph::new(text_value)
            .centered()
            .block(block)
            .style(style);

        frame.render_widget(Clear, *area);
        frame.render_widget(paragraph, *area);
    }
}
