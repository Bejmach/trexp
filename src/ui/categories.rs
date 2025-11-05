use std::collections::HashMap;

use ratatui::{layout::Rect, style::Stylize, symbols::border, text::Line, widgets::{Block, Clear, List, ListItem, Padding}};

use crate::{theme::{GaugeState, StyleData}, traits::tr_widget::TrWidget, ui::{centered_rect, gauge::build_gauge, widgets::{variant_id_to_usize, ConstraintFit, WidgetData}}};

pub struct CategoriesWidget{}

impl CategoriesWidget{
    pub fn new() -> Self{Self {  }}
}

impl TrWidget for CategoriesWidget{
    fn render(&mut self, frame: &mut ratatui::Frame, app: &crate::app::App, layout_data: &HashMap<String, Vec<Rect>>, widget: &WidgetData) {
        
        let category_id = app.additional_data.get("category_id").expect("");
        let category_id = variant_id_to_usize(category_id, app.data.categories.len());
        if let Some(id) = category_id{
            let area = match widget.constraint_fit{
                ConstraintFit::Default => layout_data.get(&widget.layout).expect("no layout with provided id").get(widget.constraint).expect("no constraint with provided id"),
                ConstraintFit::Centered { percent_x, percent_y } => &centered_rect(percent_x, percent_y, *layout_data.get(&widget.layout).expect("no layout with provided id").get(widget.constraint).expect("no constraint with provided id"))
            };

            layout_data.get(&widget.layout).expect("no layout with provided id").get(widget.constraint).expect("no constraint with provided id");

            let (style_data, focus): (&StyleData, bool) = if let Some(data) = widget.styles.get(&app.state){
                (app.app_config.styles.get(data).expect("No style with provided name"), true)
            }else if let Some(data) = widget.styles.get("_"){
                (app.app_config.styles.get(data).expect("No default style provoded"), false)
            }else{
                (&StyleData::new(), false)
            };

            let mut items: Vec<ListItem> = Vec::new();
            let size: u16 = area.width - 6;
            for (i, category) in app.data.categories.iter().enumerate(){

                let label_left = format!("{}", category.name);
                let label_right = format!("{}/{} : {}", category.exp, category.next_exp, category.lvl);
                let ratio = category.exp as f32 / category.next_exp as f32;

                let state = if focus{
                    if i == id{
                        GaugeState::Focus
                    }
                    else{
                        GaugeState::Passive
                    }
                }else if i == id{
                    GaugeState::FadedFocus
                }else{
                    GaugeState::FadedPassive
                };

                items.push(ListItem::new(build_gauge(app, label_left, label_right, ratio as f32, size, state)));
            }
            let category_list = List::new(items);

            let style = style_data.to_style();

            let block = Block::bordered()
                .title(Line::from(" Categories ".bold()))
                .border_set(border::PLAIN)
                .padding(Padding::new(2, 4, 1, 1))
                .style(style);

            frame.render_widget(Clear, *area);
            frame.render_widget(category_list.block(block), *area);
        }
    }
}
