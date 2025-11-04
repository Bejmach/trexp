use std::str::FromStr;

use ratatui::style::{Color, Modifier, Style};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TrModifier{
    Bold,
    Dim,
    Reversed,
}

impl TrModifier{
    pub fn to_mod(&self) -> Modifier{
        match self{
            TrModifier::Bold => Modifier::BOLD,
            TrModifier::Dim => Modifier::DIM,
            TrModifier::Reversed => Modifier::REVERSED,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StyleData{
    pub fg: String,
    pub bg: String,
    #[serde(default = "default_modifier")]
    pub modifier: Vec<TrModifier>,
}

fn default_modifier() -> Vec<TrModifier>{Vec::new()}

impl StyleData{
    pub fn new() -> Self{
        Self { fg: "#FFFFFF".to_string(), bg: "#000000".to_string(), modifier: Vec::new() }
    }

    pub fn to_style(&self) -> Style{
        let mut style = Style::default()
            .fg(Color::from_str(&self.fg).expect("Wrong color"))
            .bg(Color::from_str(&self.bg).expect("Wrong color"));

        for modyfier in self.modifier.iter(){
            let patch = Style::default().add_modifier(modyfier.to_mod());
            style = style.patch(patch);
        }

        style
    } 
}

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
                .bg(Color::Rgb(0, 0, 0)) 
                .add_modifier(Modifier::BOLD),

            focus: Style::default()
                .fg(Color::Rgb(200, 145, 70))
                .bg(Color::Rgb(10, 10, 10))
                .add_modifier(Modifier::BOLD),

            passive: Style::default()
                .fg(Color::Gray)
                .bg(Color::Rgb(25, 25, 25))
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
    FadedPassive,
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
    pub faded_passive: Style,
}

impl GaugeStyle {
    pub fn dark_theme() -> Self {
        Self {
            fill_char: '=',
            empty_char: ' ',
            border_char: '|',

            margin_left: 3,
            margin_right: 4,

            focus: Style::default()
                .fg(Color::Rgb(220, 165, 90))
                .bg(Color::Rgb(10, 10, 10))
                .add_modifier(Modifier::BOLD | Modifier::REVERSED),

            faded_focus: Style::default()
                .fg(Color::White)
                .bg(Color::Rgb(10, 10, 10))
                .add_modifier(Modifier::BOLD),
        
            passive: Style::default()
                .fg(Color::Rgb(200, 145, 70))
                .bg(Color::Rgb(10, 10, 10))
                .add_modifier(Modifier::BOLD),
        
            faded_passive: Style::default()
                .fg(Color::Gray)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        }
    }
}
