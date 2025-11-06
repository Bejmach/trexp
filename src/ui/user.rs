use num_format::{Buffer, Locale};
use ratatui::{layout::{self, Constraint, Layout}, text::{Line, Span}, widgets::{Clear, Paragraph}};

use crate::{json_types::calculate_exp, theme::{GaugeState, StyleData}, traits::tr_widget::TrWidget, ui::{centered_rect, gauge::build_gauge, widgets::ConstraintFit}};

pub struct UserWidget{
    name: String,
}

impl UserWidget{
    pub fn new(name: String) -> Self{
        Self { name }
    }
}

pub fn get_stats(global_exp: u64, base: u32, power: f32) -> (u32, u64, u64){
    let mut lvl: u32 = 1;
    let mut required_exp = base as u64;
    let mut cur_exp = global_exp;
    

    while cur_exp >= required_exp{
        lvl += 1;
        cur_exp -= required_exp;
        required_exp = calculate_exp(lvl, power, base) as u64;
    }

    (lvl, cur_exp, required_exp)
}

impl TrWidget for UserWidget{
    fn render(&self, frame: &mut ratatui::Frame, app: &crate::app::App, layout_data: &std::collections::HashMap<String, Vec<ratatui::prelude::Rect>>, widget: &super::widgets::WidgetData) {
        let area = match widget.constraint_fit{
            ConstraintFit::Default => layout_data.get(&widget.layout).expect("no layout with provided id").get(widget.constraint).expect("no constraint with provided id"),
            ConstraintFit::Centered { percent_x, percent_y } => &centered_rect(percent_x, percent_y, *layout_data.get(&widget.layout).expect("no layout with provided id").get(widget.constraint).expect("no constraint with provided id"))
        };

        let mut global_exp = 0;
        for category in app.data.categories.iter(){
            global_exp += category.exp_sum;
        }

        let style_data: &StyleData = if let Some(data) = widget.styles.get(&app.state){
                app.app_config.styles.get(data).expect("No style with provided name")
            }else if let Some(data) = widget.styles.get("_"){
                app.app_config.styles.get(data).expect("No default style provoded")
            }else{
                &StyleData::new()
            };

        let style = style_data.to_style();
        
        let (lvl, cur_exp, required_exp) = get_stats(global_exp, app.app_config.base_exp, app.app_config.exp_power);
        let ratio = cur_exp as f32 / required_exp as f32;

        let layout = Layout::default()
            .direction(layout::Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(40),
                Constraint::Percentage(30),
                Constraint::Percentage(30)
            ])
            .split(*area);

        let bottom_paragraph = Layout::default()
            .direction(layout::Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ])
            .split(layout[2]);

        let name_paragraph = Paragraph::new(self.name.clone())
            .style(style)
            .left_aligned();
        let gauge_paragraph = Paragraph::new(build_gauge(app, String::new(), String::new(), ratio, area.width, GaugeState::Passive));
        let lvl_paragraph = Paragraph::new(format!("lvl {}", lvl))
            .style(style)
            .left_aligned();

        let mut buf = Buffer::default();
        buf.write_formatted(&cur_exp, &Locale::en);
        let cur_exp_str = buf.as_str().to_string();

        buf.write_formatted(&required_exp, &Locale::en);
        let required_exp_str = buf.as_str().to_string();

        let exp_paragraph = Paragraph::new(format!("{} of {}", cur_exp_str, required_exp_str))
            .style(style)
            .right_aligned();

        frame.render_widget(Clear, *area);
        frame.render_widget(name_paragraph, layout[0]);
        frame.render_widget(gauge_paragraph, layout[1]);
        frame.render_widget(lvl_paragraph, bottom_paragraph[0]);
        frame.render_widget(exp_paragraph, bottom_paragraph[1]);
    }
}
