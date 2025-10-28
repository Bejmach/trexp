use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub active: Style,
    pub focus: Style,
    pub passive: Style,
    pub faded_selection: Style,
    pub selection: Style,
    pub floating: Style,
    pub error: Style,
    pub help_text: Style,
    pub help_key: Style,
    pub gauge_style: GaugeStyle,
}

impl Theme {
    pub fn dark_theme() -> Self {
        Theme {
            active: Style::default()
                .fg(Color::Rgb(220, 165, 90))
                .bg(Color::Black) 
                .add_modifier(Modifier::BOLD),

            focus: Style::default()
                .fg(Color::Rgb(200, 145, 70))
                .bg(Color::Rgb(5, 5, 5))
                .add_modifier(Modifier::BOLD),

            passive: Style::default()
                .fg(Color::Gray)
                .bg(Color::Rgb(10, 10, 10))
                .add_modifier(Modifier::DIM),

            selection: Style::default()
                .fg(Color::Rgb(220, 165, 90))
                .bg(Color::Rgb(10, 10, 10))
                .add_modifier(Modifier::BOLD | Modifier::REVERSED),
            
            faded_selection: Style::default()
                .fg(Color::Rgb(220, 165, 90))
                .bg(Color::Rgb(30, 30, 30))
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
                .fg(Color::Rgb(200, 140, 130))
                .bg(Color::Rgb(25, 25, 25))
                .add_modifier(Modifier::BOLD),

            gauge_style: GaugeStyle::dark_theme(),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum GaugeState{
    Focus,
    FadedFocus,
    Passive,
}

pub struct GaugeStyle {
    pub fill_char: char,
    pub empty_char: char,
    pub border_char: char,

    pub margin_left: u16,
    pub margin_right: u16,

    pub focus: Style,
    pub faded_focus: Style,
    pub passive: Style,
}

impl GaugeStyle {
    pub fn dark_theme() -> Self {
        Self {
            fill_char: '=',
            empty_char: ' ',
            border_char: '|',

            margin_left: 15,
            margin_right: 20,

            passive: Style::default()
                .fg(Color::Rgb(220, 165, 90))
                .bg(Color::Rgb(10, 10, 10))
                .add_modifier(Modifier::BOLD),

            focus: Style::default()
                .fg(Color::Rgb(220, 165, 90))
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD | Modifier::REVERSED),

            faded_focus: Style::default()
                .fg(Color::Rgb(220, 165, 90))
                .bg(Color::Rgb(30, 30, 30))
                .add_modifier(Modifier::BOLD),
        }
    }
}
