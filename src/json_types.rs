use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

pub fn calculate_exp(lvl: u32, power: f32, base: u32) -> u32{
    (base as f32 * (lvl as f32).powf(power)) as u32
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
    pub fn get_category_uid_mut(&mut self, uid: u64) -> Option<&mut Category>{
        for category in self.categories.iter_mut(){
            if category.get_uid() == uid{
                return Some(category);
            }
        }
        None
    }
    pub fn get_category_uid(&mut self, uid: u64) -> Option<&Category>{
        for category in self.categories.iter(){
            if category.get_uid() == uid{
                return Some(category);
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize)]
pub struct Category{
    unique_id: u64,
    pub name: String,
    pub exp_sum: u64,
    pub exp: u32,
    pub next_exp: u32,
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
            exp_sum: 0,
            exp: 0,
            next_exp: 0,
            lvl: 1,
            tasks: Vec::new(),
            milestones: Vec::new(),
        }
    }
    pub fn init(name: &str, base: u32) -> Self{
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH).expect("");

        let timestamp = now.as_secs();

        Self {
            unique_id: timestamp,
            name: name.to_string(),
            exp_sum: 0,
            exp: 0,
            next_exp: base,
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

    pub fn increase_exp(&mut self, exp: u32, power: f32, base: u32){
        self.exp_sum += exp as u64;
        self.exp += exp;
        self.lvl_up(power, base);
    }
    pub fn lvl_up(&mut self, power: f32, base: u32){
        if self.exp > self.next_exp{
            self.exp = self.exp - self.next_exp;
            self.lvl += 1;
            self.next_exp = calculate_exp(self.lvl, power, base)
        }
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
