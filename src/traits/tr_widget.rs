use std::collections::HashMap;

use ratatui::{layout::Rect, Frame};

use crate::{app::App, ui::widgets::{ConstraintFit, WidgetData}};

pub trait TrWidget {
    fn render(&self, frame: &mut Frame, app: &App, layout_data: &HashMap<String, Vec<Rect>>, data: &WidgetData);
}
