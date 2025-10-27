use std::io;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Data{
    pub categories: Vec<Category>,
}

impl Data{
    pub fn new() -> Self{
        Self {
            categories: Vec::new()
        }
    }
    pub fn add_category(&mut self, category: Category) -> Result<(), ()>{
        for old_category in self.categories.iter(){
            if old_category.name == category.name{
                return Err(());
            }
        }
        self.categories.push(category);
        Ok(())
    }
    pub fn move_category(&mut self, id: usize, by: i32) -> Result<(), ()>{
        if (id as i32 + by < 0) || (id as i32 + by >= self.categories.len() as i32){
            return Err(());
        }
        self.categories.swap(id, (id as i32 + by) as usize);
        Ok(())
    }
    pub fn remove_category(&mut self, id: usize) -> Result<(), ()>{
        if id >= self.categories.len(){
            return Err(());
        }
        self.categories.remove(id);
        Ok(())
    }
    pub fn get_category_mut(&mut self, id: usize) -> Option<&mut Category>{
        self.categories.get_mut(id)
    }
    pub fn get_category(&self, id: usize) -> Option<&Category>{
        self.categories.get(id)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Category{
    pub name: String,
    pub exp: u32,
    pub exp_to_next_lvl: u32,
    pub lvl: u32,
    pub tasks: Vec<Task>,
    pub milestones: Vec<Milestone>,
}

impl Category{
    pub fn new() -> Self{
        Self {
            name: String::new(),
            exp: 0,
            exp_to_next_lvl: 1,
            lvl: 0,
            tasks: Vec::new(),
            milestones: Vec::new(),
        }
    }
    pub fn init(name: &str) -> Self{
        Self {
            name: name.to_string(),
            exp: 0,
            exp_to_next_lvl: 1,
            lvl: 0,
            tasks: Vec::new(),
            milestones: Vec::new(),
        }
    }
    pub fn set_name(&mut self, name: String){
        self.name = name;
    }
    pub fn move_task(&mut self, id: usize, by: i32) -> Result<(), ()>{
        if (id as i32 + by < 0) || (id as i32 + by >= self.tasks.len() as i32) {
            return Err(());
        }
        self.tasks.swap(id, (id as i32 + by) as usize);
        return Ok(());
    }
    pub fn add_task(&mut self, task: Task) -> Result<(), ()>{
        for old_task in self.tasks.iter(){
            if old_task.name == task.name{
                return Err(());
            }
        }
        self.tasks.push(task);
        Ok(())
    }
    pub fn remove_task(&mut self, id: usize) -> Result<(), ()>{
        if id >= self.tasks.len(){
            return Err(());
        }
        self.tasks.remove(id);
        Ok(())
    }

    pub fn get_task(&self, id: usize) -> Option<&Task>{
        self.tasks.get(id)
    }

    pub fn move_milestone(&mut self, id: usize, by: i32) -> Result<(), ()>{
        if (id as i32 + by < 0) || (id as i32 + by >= self.milestones.len() as i32) {
            return Err(());
        }
        self.milestones.swap(id, (id as i32 + by) as usize);
        return Ok(());
    }
    pub fn add_milestone(&mut self, milestone: Milestone) -> Result<(), ()>{
        for old_milestone in self.milestones.iter(){
            if old_milestone.name == milestone.name{
                return Err(());
            }
        }
        self.milestones.push(milestone);
        Ok(())
    }
    pub fn remove_milestone(&mut self, id: usize) -> Result<(), ()>{
        if id >= self.milestones.len(){
            return Err(());
        }
        self.milestones.remove(id);
        Ok(())
    }

    pub fn get_milestone(&self, id: usize) -> Option<&Milestone>{
        self.milestones.get(id)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Task{
    pub name: String,
    pub exp_reward: u32,
}

impl Task{
    pub fn new() -> Self{
        Self {
            name: String::new(),
            exp_reward: 0,
        }
    }
    pub fn init(name: String, exp_reward: u32) -> Self{
        Self {
            name,
            exp_reward,
        }
    }
    pub fn set_name(&mut self, name: String){
        self.name = name;
    }
    pub fn set_exp_revard(&mut self, exp_reward: u32){
        self.exp_reward = exp_reward;
    }
}

#[derive(Serialize, Deserialize)]
pub struct Milestone{
    pub name: String,
    pub exp_reward: u32,
}

impl Milestone{
    pub fn new() -> Self{
        Self {
            name: String::new(),
            exp_reward: 0,
        }
    }
    pub fn init(name: String, exp_reward: u32) -> Self{
        Self {
            name,
            exp_reward,
        }
    }
    pub fn set_name(&mut self, name: String){
        self.name = name;
    }
    pub fn set_exp_revard(&mut self, exp_reward: u32){
        self.exp_reward = exp_reward;
    }
}
