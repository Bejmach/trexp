use std::collections::HashMap;

use ratatui::{layout::Rect, Frame};
use serde::{Deserialize, Serialize};

use crate::{json_types::{self, Category, Data}, layout_conf::{to_layouts, LayoutNode}, theme::{StyleData, Theme}, traits::tr_widget::TrWidget, ui::{render_error, render_result, widgets::WidgetData}, wild_type::{Generic, Variant}};

pub enum AppCommands{
    Undefined,
    Quit,
    State(String),
    Toggle(String),
    // layout_id, constraint, new_value
    Resize(String, usize, String),
    Error(String),
    Result(String),
    Set(String, Variant),
    Change(String, i64),
    Remove(String),
    OpenBuffer(String, InputMode),
    CloseBuffer,
    SaveBuffer,
    AddCategory(String),
}

impl AppCommands{
    fn from_str(value: String) -> Self{
        let value = value.trim();
        if let Some(split_id) = value.find("("){
            
            let name: &str = &value.to_lowercase()[0..split_id];
            let name = name.trim();

            let params: Vec<&str> = value[split_id+1..value.len()-1].split(",").collect();

            return match name{
                "toggle" => {
                    let layout_id = params.get(0).expect("").trim().to_string();
                    AppCommands::Toggle(layout_id)
                },
                "resize" => {
                    let layout_id = params.get(0).expect("").trim().to_string();
                    let constraint_id = params.get(1).expect("").trim().parse::<usize>().expect("");
                    let new_value = params.get(2).expect("").trim().to_string();
                    AppCommands::Resize(layout_id, constraint_id, new_value)
                },
                "state" => {
                    let state = params.get(0).expect("").trim().to_string();
                    AppCommands::State(state)
                },
                "errot" => {
                    let error_message = params.get(0).expect("").trim().to_string();
                    AppCommands::Error(error_message)
                }
                "result" => {
                    let result_message = params.get(0).expect("").trim().to_string();
                    AppCommands::Result(result_message)
                }
                "set" => {
                    let key = params.get(0).expect("").trim().to_string();
                    let value =  params.get(1).expect("").trim().to_string();
                    AppCommands::Set(key, Variant::from_string(&value, &Generic::Any))
                }
                "change" => {
                    let key = params.get(0).expect("").trim().to_string();
                    let value =  params.get(1).expect("").trim().to_string().parse::<i64>().expect("");
                    AppCommands::Change(key, value)
                }
                "remove" => {
                    let key = params.get(0).expect("").trim().to_string();
                    AppCommands::Remove(key)
                }
                "openbuffer" => {
                    let name = params.get(0).expect("").trim().to_string();
                    let mode = InputMode::from_str(params.get(1).expect("").to_string());
                    AppCommands::OpenBuffer(name, mode)
                }
                "addcategory" => {
                    let name = params.get(0).expect("").trim().to_string();
                    AppCommands::AddCategory(name)
                }
                _ => AppCommands::Undefined,
            }
        }
        else{
            let command: &str = &value.trim().to_lowercase();
            
            return match command{
                "quit" => AppCommands::Quit,
                "closebuffer" => AppCommands::CloseBuffer,
                "savebuffer" => AppCommands::SaveBuffer,
                _ => AppCommands::Undefined,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum AppComponent{
    Categories,
    Tasks,
    Milestones,
    Timers,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HelpData{
    pub command: String,
    pub info: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig{
    pub layouts: Vec<LayoutNode>,
    pub states: Vec<String>,
    pub keybinds: HashMap<String, HashMap<String, HelpData>>,
    pub widgets: Vec<WidgetData>,
    pub styles: HashMap<String, StyleData>,
    
    pub values: HashMap<String, String>,

    #[serde(default = "default_exp_power")]
    pub exp_power: f32,
}
pub fn default_exp_power() -> f32{0.85}

impl AppConfig{
    pub fn new() -> Self{
        Self {
            layouts: Vec::new(),
            states: Vec::new(),
            keybinds: HashMap::new(),
            widgets: Vec::new(),
            styles: HashMap::new(),
            values: HashMap::new(),
            exp_power: 0.85,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum InputMode{
    Undefined,
    Text,
    Number,
}

impl InputMode{
    pub fn from_str(value: String) -> InputMode{
        let value = value.to_lowercase();
        let value = value.trim();
        match value{
            "text" => InputMode::Text,
            "number" => InputMode::Number,
            _ => InputMode::Undefined,
        }
    }
}

pub struct App{
    pub exit: bool,
    pub state: String,

    pub data: json_types::Data,
    pub app_config: AppConfig,
    //pub timers: Vec<Timer>,
    
    pub theme: Theme,
    
    pub global_lvl: u32,
    pub global_exp: u32,

    pub input_mode: InputMode,
    pub buffer_name: Option<String>,
    pub input_buffer: String,

    pub result_message: String,
    pub error_message: String,

    pub additional_data: HashMap<String, Variant>
}

impl App{
    pub fn new() -> Self{
        Self {
            exit: false,
            state: String::new(),
            data: Data::new(),
            app_config: AppConfig::new(),
            theme: Theme::dark_theme(),
            global_lvl: 0,
            global_exp: 0,
            input_mode: InputMode::Undefined,
            buffer_name: None,
            input_buffer: String::new(),
            result_message: String::new(),
            error_message: String::new(),
            additional_data: HashMap::new()
        }
    }
    pub fn init(&mut self){
        for (key, value) in self.app_config.values.clone().into_iter(){
            self.additional_data.insert(key, Variant::from_string(&value, &Generic::Any));
        }
    }

    pub fn run_command_string(&mut self, commands: String){
        for command in commands.split(';'){
            self.run_command(&AppCommands::from_str(command.to_string()));
        }
    }
    pub fn run_command(&mut self, command: &AppCommands){
        match command{
            AppCommands::Quit => {
                self.exit = true;
            },
            AppCommands::State(state) => {
                self.set_state(state.to_string());
            },
            AppCommands::Toggle(widget_id) => {
                self.toggle_widget(widget_id.to_string());
            },
            AppCommands::Resize(layout_id, constraint, new_value) => {
                self.resize_constraint(layout_id.to_string(), *constraint, new_value.to_string());
            },
            AppCommands::Error(error_message) => {
                self.error_message = error_message.to_string();
            }
            AppCommands::Result(result_message) => {
                self.result_message = result_message.to_string();
            }
            AppCommands::Set(key, value) => {
                self.set_data(key.to_string(), value.clone());
            }
            AppCommands::Change(key, value) => {
                self.change_data(key.to_string(), *value);
            }
            AppCommands::Remove(key) => {
                self.remove_data(key.to_string());
            }
            AppCommands::OpenBuffer(name, mode) => {
                self.open_buffer(name.to_string(), mode.clone());
            }
            AppCommands::CloseBuffer => {
                self.close_buffer();
            }
            AppCommands::SaveBuffer => {
                self.save_buffer();
            }
            AppCommands::AddCategory(name) => {
                self.add_category(name.to_string());
            }
            _ => {}
        }
    }

    pub fn load_config(&mut self, config: AppConfig){
        self.app_config = config;
        self.state = self.app_config.states.first().expect("No states provided").to_string();
    }

    pub fn render_widgets(&mut self, frame: &mut Frame){
        let layout_data: HashMap<String, Vec<Rect>> = to_layouts(&self.app_config.layouts, frame.area());

        for widget in self.app_config.widgets.iter(){
            if !widget.visible{
                continue;
            }
            if let Some(widget_box) = widget.widget_type.to_widget(){
                widget_box.render(frame, self, &layout_data, &widget);
            }
        }

        if self.result_message != String::new(){
            render_result(self, frame, 60, 40, frame.area());
        }
        if self.error_message != String::new(){
            render_error(self, frame, 60, 40, frame.area());
        }
    }

    pub fn open_buffer(&mut self, name: String, mode: InputMode){
        self.input_mode = mode;
        self.buffer_name = Some(name);
        self.input_buffer = String::new();
    }

    pub fn close_buffer(&mut self){
        self.input_mode = InputMode::Undefined;
        self.buffer_name = None;
        self.input_buffer = String::new();
    }

    pub fn save_buffer(&mut self){
        if let Some(name) = &self.buffer_name{
            match self.input_mode{
                InputMode::Number => self.set_data(name.to_string(), Variant::from_string(&self.input_buffer, &Generic::Int)),
                InputMode::Text => self.set_data(name.to_string(), Variant::from_string(&self.input_buffer, &Generic::Str)),
                _ => {}
            }
        }
    }

    pub fn add_category(&mut self, name: String){
        if name.starts_with("$"){
            let name = name[1..].to_string();
            if Some(name.clone()) == self.buffer_name{
                for category in self.data.categories.iter(){
                    if category.name == self.input_buffer{
                        self.error_message = "Category already exist".to_string();
                        return;
                    }
                }
                let _ = self.data.add_category(Category::init(&self.input_buffer));
                self.result_message = "Category succesfully added".to_string();
            }
            else if let Some(Variant::Str(value)) = self.additional_data.get(&name){
                for category in self.data.categories.iter(){
                    if category.name == *value{
                        self.error_message = "Category already exist".to_string();
                        return;
                    }
                }
                let _ = self.data.add_category(Category::init(value));
                self.result_message = "Category succesfully added".to_string();
            }
            else{
                self.error_message = "Couldn't add category".to_string();
            }
        }
        else{
            self.error_message = "Category name needs to be a param, not literal".to_string();
        }
    }

    pub fn set_data(&mut self, key: String, value: Variant){
        self.additional_data.insert(key, value);
    }
    pub fn change_data(&mut self, key: String, value: i64){
        if let Some(old_value) = self.additional_data.get(&key){
            match old_value{
                Variant::Int(old_int) => {
                    self.additional_data.insert(key, Variant::from_string(&(old_int + value).to_string(), &Generic::Int));
                },
                _ => {}
            }
        }
    }
    pub fn remove_data(&mut self, key: String){
        self.additional_data.remove(&key);
    }

    pub fn resize_constraint(&mut self, layout_id: String, constraint: usize, new_value: String){
        if let Ok(id) = layout_id.parse::<usize>(){
            if let Some(layout) = self.app_config.layouts.get_mut(id){
                if layout.constraints.len() < constraint{
                    layout.constraints[constraint] = new_value;
                }
            }
        }
        else{
            for layout in self.app_config.layouts.iter_mut(){
                if layout.id == layout_id{
                    if layout.constraints.len() < constraint{
                        layout.constraints[constraint] = new_value;
                        break;
                    }
                }
            }
        }

        
    }
    pub fn set_state(&mut self, state: String){
        if state.starts_with("$"){
            if let Some(Variant::Str(val_state)) = self.additional_data.get(&state[1..]){
                if self.app_config.states.contains(val_state){
                    self.state = val_state.to_string();
                }
            }
        }
        else if self.app_config.states.contains(&state){
            self.state = state;
        }
    }

    pub fn toggle_widget(&mut self, widget_id: String){
        if let Ok(id) = widget_id.parse::<usize>(){
            if let Some(widget) = self.app_config.widgets.get_mut(id){
                widget.visible = !widget.visible;
            }
        }
        else{
            for widget in self.app_config.widgets.iter_mut(){
                if widget.id == widget_id{
                    widget.visible = !widget.visible;
                }
            }
        }
    }
}
