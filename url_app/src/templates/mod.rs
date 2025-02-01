use lazy_static::lazy_static;

lazy_static! {
    pub static ref INDEX_TEMPLATE: String = include_str!(r"index.html")
        .to_string();
}