use crate::json_types::Category;

pub struct Timer{
    pub category_id: u64,
    pub category_name: String,
    start_time: std::time::Instant,
}

impl Timer{
    pub fn new(category: &Category) -> Self{
        Self{
            category_id: category.get_uid().clone(),
            category_name: category.name.clone(),
            start_time: std::time::Instant::now(),
        }
    }

    pub fn get_second(&self) -> u32{
        (self.start_time.elapsed().as_secs() % 60) as u32
    }

    pub fn get_minutes(&self) -> u32{
        (self.start_time.elapsed().as_secs() / 60) as u32
    }
}
