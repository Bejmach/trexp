pub struct Timer{
    category_name: String,
    start_time: std::time::Instant,
}

impl Timer{
    pub fn new(category_name: String) -> Self{
        Self{
            category_name,
            start_time: std::time::Instant::now(),
        }
    }
}
