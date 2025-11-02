use std::collections::HashMap;

use ratatui::{layout::Rect, style::Stylize, symbols::border, text::{Line, Span}, widgets::{Block, List, ListItem, Padding}};

use crate::{theme::StyleData, traits::tr_widget::TrWidget, ui::{centered_rect, widgets::{variant_id_to_usize, ConstraintFit, WidgetData}}};

pub struct TaskWidget{}

impl TaskWidget{
    pub fn new() -> Self{Self {  }}
}

impl TrWidget for TaskWidget{
    fn render(&self, frame: &mut ratatui::Frame, app: &crate::app::App, layout_data: &HashMap<String, Vec<Rect>>, widget: &WidgetData) {
        let mut items: Vec<ListItem> = Vec::new();

        let category_id = app.additional_data.get("category_id").expect("");
        let category_id = variant_id_to_usize(category_id, app.data.categories.len());

        let (style_data, focus): (&StyleData, bool) = if let Some(data) = widget.styles.get(&app.state){
            (app.app_config.styles.get(data).expect("No style with provided name"), true)
        }else if let Some(data) = widget.styles.get("_"){
            (app.app_config.styles.get(data).expect("No default style provoded"), false)
        }else{
            (&StyleData::new(), false)
        };

        if let Some(category_id) = category_id{
            if let Some(category) = app.data.get_category(category_id){
                let area = match widget.constraint_fit{
                    ConstraintFit::Default => layout_data.get(&widget.layout).expect("no layout with provided id").get(widget.constraint).expect("no constraint with provided id"),
                    ConstraintFit::Centered { percent_x, percent_y } => &centered_rect(percent_x, percent_y, *layout_data.get(&widget.layout).expect("no layout with provided id").get(widget.constraint).expect("no constraint with provided id"))
            };
                let task_id = app.additional_data.get("task_id").expect("");
                let task_id = variant_id_to_usize(task_id, category.tasks.len());

                if let Some(task_id) = task_id{
                    for (i, task) in category.tasks.iter().enumerate(){
                        let task_text = format!("{} [+{} XP]", task.name, task.exp_reward);

                        let style = if focus {
                            app.theme.selection
                        }else if i == task_id{
                            app.theme.faded_selection
                        }else{
                            app.theme.passive
                        };

                        items.push(ListItem::new(Line::from(Span::styled(
                            task_text,
                            style
                        ))));
                    }
                }
                let task_list = List::new(items);

                let block = Block::bordered()
                    .title(Line::from(" Tasks ".bold()))
                    .border_set(border::PLAIN)
                    .padding(Padding::new(2, 4, 1, 1))
                    .style(style_data.to_style());

                frame.render_widget(task_list.block(block), *area);
            }
        }
    }
}
