use std::{collections::HashMap};

use ratatui::{layout::{Constraint, Direction, Layout, Rect}};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum LayoutDirection{
    Horizontal,
    Vertical,
}

impl LayoutDirection{
    fn to_direction(&self) -> Direction{
        match self{
            LayoutDirection::Horizontal => Direction::Horizontal,
            LayoutDirection::Vertical => Direction::Vertical,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LayoutNode{
    pub id: String,
    pub direction: LayoutDirection,
    
    #[serde(default = "default_margin")]
    pub margin: u16,
    pub constraints: Vec<String>,
    
    #[serde(default = "default_parent")]
    pub parent: String,
}

fn default_margin() -> u16{0}
fn default_visible() -> bool{true}
fn default_parent() -> String{String::new()}

pub fn get_area(layout_data: &HashMap<String, Vec<Rect>>, raw_value: String) -> &Rect{
    if let Some(data) = raw_value.split_once(".") {
        let id: String = data.0.to_string();
        let value: usize = data.1.parse::<usize>().expect("couldnt parse value");

        return layout_data.get(&id).expect("Cant find leyout with provided id").get(value).expect("layout does not provide enought constraints");
    }
    else{
        let id: String = raw_value;
        let value: usize = 0;

        return layout_data.get(&id).expect("Cant find leyout with provided id").get(value).expect("layout does not provide enought constraints");
    }

    
}

pub fn vec_to_constraints(data: Vec<String>) -> Vec<Constraint>{
    let mut constraints: Vec<Constraint> = Vec::with_capacity(data.len());
    for value in data.iter(){
        if let Some(single_data) = value.split_once("."){
            let parsed_length = single_data.1.parse::<u16>().expect("couldnt parse value");
            match single_data.0{
                "p" => constraints.push(Constraint::Percentage(parsed_length)),
                "l" => constraints.push(Constraint::Length(parsed_length)),
                _ => constraints.push(Constraint::Percentage(parsed_length)),
            }
        }
        else{
            let parsed_length = value.parse::<u16>().expect("couldnt parse value");
            constraints.push(Constraint::Percentage(parsed_length));
        }
    }
    constraints
}

pub fn to_layouts(layout_nodes: &Vec<LayoutNode>, frame_area: Rect) -> HashMap<String, Vec<Rect>>{
    let mut layout_data: HashMap<String, Vec<Rect>> = HashMap::new();
    layout_data.insert("frame".to_string(), vec![frame_area]);

    for layout in layout_nodes.iter(){
        let split_area: &Rect = if layout.parent == String::new(){
            &frame_area
        }else{
            get_area(&layout_data, layout.parent.to_string())
        };

        let rect_vec = Layout::default()
            .direction(layout.direction.to_direction())
            .constraints(vec_to_constraints(layout.constraints.clone()))
            .split(*split_area).to_vec();

        layout_data.insert(layout.id.clone(), rect_vec);
    }

    layout_data
}
