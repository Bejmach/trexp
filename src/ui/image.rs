use std::collections::HashMap;

use ratatui::{layout::Rect, widgets::Clear, Frame};
use ratatui_image::{picker::Picker, protocol::{Protocol, StatefulProtocol}, Image, StatefulImage};

use crate::{app::App, traits::tr_widget::{TrWidget}, ui::{centered_rect, widgets::{ConstraintFit, WidgetData}}};

pub struct ImageWidget{
    image: StatefulProtocol,
}

impl ImageWidget{
    pub fn new(path: String) -> Self{
        let picker = Picker::from_fontsize((8, 12));

        let dyn_img = image::ImageReader::open(path).expect("wrong path").decode().expect("cant decode");

        let image = picker.new_resize_protocol(dyn_img);

        Self { image }
    }
}

impl TrWidget for ImageWidget{
    fn render(&mut self, frame: &mut Frame, _app: &App, layout_data: &HashMap<String, Vec<Rect>>, data: &WidgetData) {
        let area = match data.constraint_fit{
                ConstraintFit::Default => layout_data.get(&data.layout).expect("no layout with provided id").get(data.constraint).expect("no constraint with provided id"),
                ConstraintFit::Centered { percent_x, percent_y } => &centered_rect(percent_x, percent_y, *layout_data.get(&data.layout).expect("no layout with provided id").get(data.constraint).expect("no constraint with provided id"))
            };

        let image = StatefulImage::new();

        frame.render_widget(Clear, *area);
        frame.render_stateful_widget(image, *area, &mut self.image);

        let _ = self.image.last_encoding_result().unwrap();
    }
}
