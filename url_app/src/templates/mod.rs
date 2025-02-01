use std::fs;

pub struct Templates {
    pub index: String
}

impl Templates {
    pub fn new() -> Self {
        Self {
            index: fs::read_to_string("src/templates/index.html")
                .expect("Failed to load html")
        }
    }
}