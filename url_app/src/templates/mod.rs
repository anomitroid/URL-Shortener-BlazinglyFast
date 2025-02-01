pub struct Templates {
    pub index: &'static str,
}

impl Templates {
    pub fn new() -> Self {
        Self {
            index: include_str!(r"index.html"),
        }
    }
}