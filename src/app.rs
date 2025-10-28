use std::fs::File;
use std::io::{self, Read, Write};

use crate::json_types::{self, Category, Data, Milestone, Task};
use crate::theme::Theme;
use crate::timer::Timer;

#[derive(PartialEq, Clone, Copy)]
pub enum AppComponent{
    Categories,
    Tasks,
    Milestones,
    Timers,
}

#[derive(PartialEq, Clone, Copy)]
pub enum AppState{
    Main,
    Categories,
    CreateCategory,
    DeleteCategory,
    EditCategory,
    Tasks,
    CreateTask,
    DeleteTask,
    EditTask,
    Milestones,
    CreateMilestone,
    DeletaMilestone,
    EditMilestone,
    Timers,
    Save,
    Exit,
}

#[derive(PartialEq, Clone, Copy)]
pub enum AppEdit{
    None,
    Name,
    Exp,
    Date,
}

pub struct AppConfig{
    pub exp_power: f32,
}
impl AppConfig{
    pub fn new() -> Self{
        Self {
            exp_power: 0.85
        }
    }
}

pub struct App{
    pub exit: bool,
    pub state: AppState,
    pub app_grid: Vec<Vec<AppComponent>>,
    pub app_size: (usize, usize),
    pub grid_cursor: (usize, usize),
    pub cur_category: u32,
    pub cur_task: u32,
    pub cur_milestone: u32,
    pub cur_timer: u32,
    pub data: json_types::Data,
    pub app_config: AppConfig,
    pub timers: Vec<Timer>,

    pub theme: Theme,
    
    pub global_lvl: u32,
    pub global_exp: u32,

    pub cur_edit: AppEdit,
    pub edit_name: String,
    pub edit_exp: String,

    pub result_message: String,
    pub error_message: String,
}

impl App{
    pub fn new() -> Self{
        let app_grid: Vec<Vec<AppComponent>> = vec![
            vec![AppComponent::Categories, AppComponent::Tasks],
            vec![AppComponent::Categories, AppComponent::Milestones],
            vec![AppComponent::Timers, AppComponent::Timers]
        ];

        Self {
            exit: false,
            state: AppState::Main,
            app_grid,
            app_size: (2, 3),
            grid_cursor: (0, 0),
            cur_category: 0,
            cur_task: 0,
            cur_milestone: 0,
            cur_timer: 0,
            data: Data::new(),
            app_config: AppConfig::new(),
            timers: Vec::new(),

            theme: Theme::dark_theme(),

            global_lvl: 0,
            global_exp: 0,

            cur_edit: AppEdit::None,
            edit_name: String::new(),
            edit_exp: String::new(),

            result_message: String::new(),
            error_message: String::new(),
        }
    }
    /*pub fn init(app_data: Vec<(AppComponent, (u32, u32), (u32, u32))>) -> Self{
        
    }*/

    pub fn load_data(&mut self) -> io::Result<()>{
        let mut file = File::open("data.json")?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        self.data = serde_json::from_str(&content)?;

        Ok(())
    }

    pub fn save_data(&mut self) -> io::Result<()>{
        while self.timers.len() > 0{
            self.cur_timer = 0;
            let _ = self.finish_timer();
        }

        let data: String = serde_json::to_string(&self.data)?;

        let mut file = File::create("data.json")?;
        file.write_all(data.as_bytes())?;

        Ok(())
    }

    pub fn get_cur_component(&self) -> Option<&AppComponent>{
        if let Some(inner_vec) = self.app_grid.get(self.grid_cursor.1) {
            return inner_vec.get(self.grid_cursor.0);
        }
        None
    }

    pub fn get_component(&self, position: &(usize, usize)) -> Option<&AppComponent>{
        if let Some(inner_vec) = self.app_grid.get(position.1) {
            return inner_vec.get(position.0);
        }
        None
    }

    pub fn set_state(&mut self, state: AppState){
        self.state = state;
    }

    pub fn save_category(&mut self) -> Result<(), ()>{
        let name = self.edit_name.clone();
        self.edit_name = String::new();
        self.data.add_category(Category::init(&name))?;
        
        Ok(())
    }
    pub fn delete_category(&mut self) -> Result<(), ()>{
        let _ = self.finish_timer_on_category_id();
        self.data.remove_category(self.cur_category as usize)?;
        self.cur_category = 0;
        Ok(())
    }
    pub fn edit_category(&mut self) -> Result<(), ()>{
        let name = self.edit_name.clone();
        self.edit_name = String::new();
        self.data.edit_category(self.cur_category as usize, name)?;
        Ok(())
    }
    pub fn save_task(&mut self) -> Result<(), ()>{
        if let Some(category) = self.data.get_category_mut(self.cur_category as usize){
            let name = self.edit_name.clone();
            if self.edit_exp.parse::<u32>().is_err(){
                return Err(());
            }
            let exp: u32 = self.edit_exp.parse().unwrap();
            self.edit_name = String::new();
            self.edit_exp = String::new();

            category.add_task(Task::init(name, exp))?;
            return Ok(());
        }
        Err(())
    }
    pub fn finish_task(&mut self) -> Result<(), ()>{
        if let Some(category) = self.data.get_category_mut(self.cur_category as usize){
            if let Some(task) = category.get_task(self.cur_task as usize){
                category.increase_exp(task.exp_reward, self.app_config.exp_power);
                return Ok(())
            }
        }
        Err(())
    }
    pub fn save_milestone(&mut self) -> Result<(), ()>{
        if let Some(category) = self.data.get_category_mut(self.cur_category as usize){
            let name = self.edit_name.clone();
            if self.edit_exp.parse::<u32>().is_err(){
                return Err(());
            }
            let exp: u32 = self.edit_exp.parse().unwrap();
            self.edit_name = String::new();
            self.edit_exp = String::new();

            category.add_milestone(Milestone::init(name, exp))?;
            return Ok(());
        }
        Err(())
    }
    pub fn finish_milestone(&mut self) -> Result<(), ()>{
        if let Some(category) = self.data.get_category_mut(self.cur_category as usize){
            if let Some(milestone) = category.get_milestone(self.cur_milestone as usize){
                category.increase_exp(milestone.exp_reward, self.app_config.exp_power);
                category.remove_milestone(self.cur_milestone as usize)?;
                return Ok(())
            }
        }
        Err(())
    }

    pub fn run_timer(&mut self) -> Result<(), ()>{
        if let Some(category) = self.data.get_category(self.cur_category as usize){
            for timer in self.timers.iter(){
                if timer.category_id == category.get_uid(){
                    self.error_message = "Timer for that category already exist".to_string();
                    return Err(());
                }
            }
            let timer = Timer::new(category);
            self.timers.push(timer);
            return Ok(());
        }
        Err(())
    }

    pub fn finish_timer(&mut self) -> Result<(), ()>{
        if let Some(timer) = self.timers.get(self.cur_timer as usize){
            for category in self.data.categories.iter_mut(){
                if timer.category_id == category.get_uid(){
                    category.increase_exp(timer.get_minutes(), self.app_config.exp_power);
                    break;
                }
            }
            self.timers.remove(self.cur_timer as usize);
            return Ok(());
        }
        Err(())
    }
    pub fn finish_timer_on_category_id(&mut self) -> Result<(), ()>{
        if let Some(category) = self.data.get_category_mut(self.cur_category as usize){
            for (i, timer) in self.timers.iter().enumerate(){
                if category.get_uid() == timer.category_id{
                    category.increase_exp(timer.get_minutes(), self.app_config.exp_power);
                    self.timers.remove(i as usize);
                    return Ok(());
                }
            }
        }
        Err(())
    }

    pub fn move_cursor_up(&mut self){
        let cur_component = self.get_cur_component().cloned();

        for i in 1..self.app_size.1{
            let position_y = (self.grid_cursor.1 + self.app_size.1 - i) % self.app_size.1;
            
            let new_position: (usize, usize) = (self.grid_cursor.0, position_y);
            let should_move: bool = {
                let new_component = self.get_component(&new_position).copied();
                cur_component != new_component
            };
            
            if should_move{
                self.grid_cursor = new_position;
                break;
            }
        }
    }
    pub fn move_cursor_down(&mut self){
        let cur_component = self.get_cur_component().cloned();

        for i in 1..self.app_size.1+1{
            let position_y = (self.grid_cursor.1 + i) % self.app_size.1;
            
            let new_position: (usize, usize) = (self.grid_cursor.0, position_y);
            let should_move: bool = {
                let new_component = self.get_component(&new_position).cloned();
                cur_component != new_component
            };
            
            if should_move{
                self.grid_cursor = new_position;
                break; 
            }
        }
    }
    pub fn move_cursor_right(&mut self){
        let cur_component = self.get_cur_component().cloned();

        for i in 1..self.app_size.0{
            let position_x = (self.grid_cursor.0 + i) % self.app_size.0;
            
            let new_position: (usize, usize) = (position_x, self.grid_cursor.1);
            let should_move: bool = {
                let new_component = self.get_component(&new_position).cloned();
                cur_component != new_component
            };
            
            if should_move{
                self.grid_cursor = new_position;
                break;
            }
        }
    }
    pub fn move_cursor_left(&mut self){
        let cur_component = self.get_cur_component().cloned();

        for i in 1..self.app_size.0+1{
            let position_x = (self.grid_cursor.0 + self.app_size.0 - i) % self.app_size.0;
            
            let new_position: (usize, usize) = (position_x, self.grid_cursor.1);
            let should_move: bool = {
                let new_component = self.get_component(&new_position).cloned();
                cur_component != new_component
            };
            
            if should_move{
                self.grid_cursor = new_position;
                break;
            }
        }
    }

    pub fn id_up(&mut self){
        match self.state{
            AppState::Categories => {
                if self.data.categories.len() != 0{
                    self.cur_category = (self.data.categories.len() as u32 + self.cur_category - 1) % self.data.categories.len() as u32;
                }
            },
            AppState::Tasks => {
                if let Some(category) = self.data.categories.get(self.cur_category as usize){
                    self.cur_task = (category.tasks.len() as u32 + self.cur_task - 1) % category.tasks.len() as u32;
                }
            }
            AppState::Milestones => {
                if let Some(category) = self.data.categories.get(self.cur_category as usize){
                    self.cur_milestone = (category.milestones.len() as u32 + self.cur_milestone - 1) % category.milestones.len() as u32;
                }
            }
            AppState::Timers => {
                if self.timers.len() != 0{
                    self.cur_timer = (self.timers.len() as u32 + self.cur_timer - 1) % self.data.categories.len() as u32;
                }
            }
            _ => {}
        }
    }

    pub fn id_down(&mut self){
        match self.state{
            AppState::Categories => {
                if self.data.categories.len() != 0{
                    self.cur_category = (self.cur_category + 1) % self.data.categories.len() as u32;
                }
            },
            AppState::Tasks => {
                if let Some(category) = self.data.categories.get(self.cur_category as usize){
                    self.cur_task = (self.cur_task + 1) % category.tasks.len() as u32;
                }
            }
            AppState::Milestones => {
                if let Some(category) = self.data.categories.get(self.cur_category as usize){
                    self.cur_milestone = (self.cur_milestone + 1) % category.milestones.len() as u32;
                }
            }
            AppState::Timers => {
                if self.timers.len() != 0{
                    self.cur_timer = (self.cur_timer + 1) % self.data.categories.len() as u32;
                }
            }
            _ => {}
        }
    }
}
