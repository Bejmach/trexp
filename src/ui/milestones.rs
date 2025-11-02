use std::collections::HashMap;

use ratatui::{layout::Rect, style::Stylize, symbols::border, text::{Line, Span}, widgets::{Block, List, ListItem, Padding}};

use crate::{theme::StyleData, traits::tr_widget::TrWidget, ui::{centered_rect, widgets::{variant_id_to_usize, ConstraintFit, WidgetData}}};

pub struct MilestoneWidget{}

impl MilestoneWidget{
    pub fn new() -> Self{Self {  }}
}

impl TrWidget for MilestoneWidget{
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
                let milestone_id = app.additional_data.get("milestone_id").expect("");
                let milestone_id = variant_id_to_usize(milestone_id, category.milestones.len());

                if let Some(milestone_id) = milestone_id{
                    for (i, milestone) in category.milestones.iter().enumerate(){
                        let milestone_text = format!("{} [+{} XP]", milestone.name, milestone.exp_reward);

                        let style = if focus {
                            app.theme.selection
                        }else if i == milestone_id{
                            app.theme.faded_selection
                        }else{
                            app.theme.passive
                        };

                        items.push(ListItem::new(Line::from(Span::styled(
                            milestone_text,
                            style
                        ))));
                    }
                }
                let milestone_list = List::new(items);

                let block = Block::bordered()
                    .title(Line::from(" Milestones ".bold()))
                    .border_set(border::PLAIN)
                    .padding(Padding::new(2, 4, 1, 1))
                    .style(style_data.to_style());

                frame.render_widget(milestone_list.block(block), *area);
            }
        }
    }
}
