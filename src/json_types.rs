use std::{io, time::{SystemTime, UNIX_EPOCH}};

use serde::{Deserialize, Serialize};

pub fn exp_for_lvl(lvl: u32, power: f32) -> u32{
    (20.0 * (lvl as f32).powf(power)) as u32
}

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
    pub fn edit_category(&mut self, id: usize, name: String) -> Result<(), ()>{
        for old_category in self.categories.iter(){
            if old_category.name == name{
                return Err(());
            }
        }
        if let Some(category) = self.get_category_mut(id){
            category.set_name(name);
            return Ok(());
        }
        Err(())
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
    unique_id: u64,
    pub name: String,
    pub exp: u32,
    pub exp_to_next_lvl: u32,
    pub lvl: u32,
    pub tasks: Vec<Task>,
    pub milestones: Vec<Milestone>,
}

impl Category{
    pub fn new() -> Self{
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH).expect("");

        let timestamp = now.as_secs();

        Self {
            unique_id: timestamp,
            name: String::new(),
            exp: 0,
            exp_to_next_lvl: 20,
            lvl: 1,
            tasks: Vec::new(),
            milestones: Vec::new(),
        }
    }
    pub fn init(name: &str) -> Self{
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH).expect("");

        let timestamp = now.as_secs();

        Self {
            unique_id: timestamp,
            name: name.to_string(),
            exp: 0,
            exp_to_next_lvl: 20,
            lvl: 1,
            tasks: Vec::new(),
            milestones: Vec::new(),
        }
    }
    pub fn get_uid(&self) -> u64{
        self.unique_id
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

    pub fn increase_exp(&mut self, exp: u32, power: f32){
        self.exp += exp;
        while self.exp >= self.exp_to_next_lvl{
            self.lvl_up(power);
        }
    }

    pub fn lvl_up(&mut self, power: f32){
        self.exp -= self.exp_to_next_lvl;
        self.lvl += 1;
        self.exp_to_next_lvl = exp_for_lvl(self.lvl, power);
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
