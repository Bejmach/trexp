use crate::{app::App, json_types::Category};

pub struct Timer{
    pub category_id: u64,
    pub category_name: String,
    start_time: std::time::Instant,
    timer_duration: f32,
    collected_exp: u32,
}

impl Timer{
    pub fn new(category: &Category, timer_duration: f32) -> Self{
        Self{
            category_id: category.get_uid().clone(),
            category_name: category.name.clone(),
            start_time: std::time::Instant::now(),
            timer_duration,
            collected_exp: 0,
        }
    }

    pub fn get_second(&self) -> f32{
        self.start_time.elapsed().as_secs_f32()
    }

    pub fn get_minutes(&self) -> u32{
        (self.start_time.elapsed().as_secs() / 60) as u32
    }

    pub fn is_finished(&mut self) -> bool{
        if self.start_time.elapsed().as_secs_f32() > self.timer_duration{
            self.collected_exp += 1;
            self.start_time = std::time::Instant::now();
            return true;
        }
        false
    }
}
