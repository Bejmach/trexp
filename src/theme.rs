use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub passive: Style,
    pub focus: Style,
    pub active: Style,
    pub faded_selection: Style,
    pub selection: Style,
    pub floating: Style,
    pub error: Style,
    pub help_text: Style,
    pub help_key: Style,
}

impl Theme {
    pub fn dark_theme() -> Self {
        Theme {
            passive: Style::default()
                .fg(Color::Gray)
                .bg(Color::Rgb(10, 10, 10))
                .add_modifier(Modifier::DIM),

            focus: Style::default()
                .fg(Color::Cyan)
                .bg(Color::Rgb(5, 5, 5))
                .add_modifier(Modifier::BOLD),

            active: Style::default()
                .fg(Color::Rgb(0, 255, 200))
                .bg(Color::Black) 
                .add_modifier(Modifier::BOLD),

            faded_selection: Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(0, 127, 127))
                .add_modifier(Modifier::BOLD),
            
            selection: Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),

            floating: Style::default()
                .fg(Color::White)
                .bg(Color::Rgb(20, 20, 30))
                .add_modifier(Modifier::BOLD),

            error: Style::default()
                .fg(Color::White)
                .bg(Color::Rgb(60, 0, 0))
                .add_modifier(Modifier::BOLD),

            help_text: Style::default()
                .fg(Color::Gray)
                .bg(Color::Rgb(25, 25, 25)),

            help_key: Style::default()
                .fg(Color::Rgb(80, 160, 255))
                .bg(Color::Rgb(25, 25, 25))
                .add_modifier(Modifier::BOLD),
        }
    }
}
