use std::collections::HashMap;

use ratatui::{layout::Rect, style::Stylize, symbols::border, text::{Line, Span}, widgets::{Block, Clear, List, ListItem, Padding}};

use crate::{theme::{GaugeState, StyleData}, traits::tr_widget::TrWidget, ui::{centered_rect, gauge::build_gauge, widgets::{variant_id_to_usize, ConstraintFit, WidgetData}}};

pub struct TimerWidget{
}

impl TimerWidget{
    pub fn new() -> Self{Self {  }}
}

impl TrWidget for TimerWidget{
    fn render(&mut self, frame: &mut ratatui::Frame, app: &crate::app::App, layout_data: &HashMap<String, Vec<Rect>>, widget: &WidgetData) {
        let mut items: Vec<ListItem> = Vec::new();

        let area = match widget.constraint_fit{
            ConstraintFit::Default => layout_data.get(&widget.layout).expect("no layout with provided id").get(widget.constraint).expect("no constraint with provided id"),
            ConstraintFit::Centered { percent_x, percent_y } => &centered_rect(percent_x, percent_y, *layout_data.get(&widget.layout).expect("no layout with provided id").get(widget.constraint).expect("no constraint with provided id"))
        };

        let (style_data, focus): (&StyleData, bool) = if let Some(data) = widget.styles.get(&app.state){
            (app.app_config.styles.get(data).expect("No style with provided name"), true)
        }else if let Some(data) = widget.styles.get("_"){
            (app.app_config.styles.get(data).expect("No default style provoded"), false)
        }else{
            (&StyleData::new(), false)
        };

        let timer_id = app.additional_data.get("timer_id").expect("");
        let timer_id = variant_id_to_usize(timer_id, app.timers.len());

        let size: u16 = area.width - 6;

        if let Some(timer_id) = timer_id{
            for (i, timer) in app.timers.iter().enumerate(){
                let label_left = format!("{}", timer.category_name);
                let label_right = String::new();
                let ratio = (timer.get_second()%app.app_config.timer_frequency) / app.app_config.timer_frequency;

                let state = if focus{
                    if i == timer_id{
                        GaugeState::Focus
                    }
                    else{
                        GaugeState::Passive
                    }
                }else if i == timer_id{
                    GaugeState::FadedFocus
                }else{
                    GaugeState::FadedPassive
                };

                items.push(ListItem::new(build_gauge(app, label_left, label_right, ratio, size, state)));
            }
        }
        let task_list = List::new(items);

        let block = Block::bordered()
            .title(Line::from(" Timers ".bold()))
            .border_set(border::PLAIN)
            .padding(Padding::new(2, 4, 1, 1))
            .style(style_data.to_style());

        frame.render_widget(Clear, *area);
        frame.render_widget(task_list.block(block), *area);
    }
}
