use std::collections::HashMap;

use ratatui::{layout::Rect, Frame};

use crate::{app::App, ui::widgets::WidgetData};

pub trait TrWidget {
    fn render(&mut self, frame: &mut Frame, app: &App, layout_data: &HashMap<String, Vec<Rect>>, data: &WidgetData);
}
