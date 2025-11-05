use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{traits::tr_widget::TrWidget, ui::{categories::CategoriesWidget, help::HelpWidget, image::ImageWidget, input::InputWidget, milestones::MilestoneWidget, tasks::TaskWidget, timers::TimerWidget}, wild_type::Variant};

#[derive(Serialize, Deserialize, Debug)]
pub enum WidgetTypes{
    Categories,
    Tasks,
    Milestones,
    Timers,
    Help,
    Input {buffer_name: String},
    Image {path: String},
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ConstraintFit{
    Default,
    Centered {percent_x: u16, percent_y: u16},
}

impl WidgetTypes{
    pub fn to_widget(&self) -> Option<Box<dyn TrWidget>>{
        match self{
            WidgetTypes::Categories => Some(Box::new(CategoriesWidget::new())),
            WidgetTypes::Tasks => Some(Box::new(TaskWidget::new())),
            WidgetTypes::Milestones => Some(Box::new(MilestoneWidget::new())),
            WidgetTypes::Help => Some(Box::new(HelpWidget::new())),
            WidgetTypes::Input {buffer_name} => Some(Box::new(InputWidget::new(buffer_name.to_string()))),
            WidgetTypes::Timers => Some(Box::new(TimerWidget::new())),
            WidgetTypes::Image { path } => Some(Box::new(ImageWidget::new(path.to_string()))),
            _ => None
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WidgetData{
    #[serde(default = "default_id")]
    pub id: String,
    pub widget_type: WidgetTypes,

    #[serde(default = "default_fit")]
    pub constraint_fit: ConstraintFit, 
    pub layout: String,
    pub constraint: usize,

    #[serde(default = "default_visible")]
    pub visible: bool,

    pub styles: HashMap<String, String>,
}
pub fn default_id() -> String{"def".to_string()}
pub fn default_visible() -> bool{true}
pub fn default_fit() -> ConstraintFit{ConstraintFit::Default}

pub fn variant_id_to_usize(id: &Variant, array_len: usize) -> Option<usize>{
    if let Variant::Int(id) = id{
        if array_len == 0{
            return Some(0);
        }
        else if *id < 0{
            return Some((array_len as i64 - 1 + (id + 1) % array_len as i64) as usize);
        }else{
            return Some(*id as usize%array_len);
        }
    }
    None
}
