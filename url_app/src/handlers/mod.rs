pub mod index;
pub mod short_urls;
pub mod redirect;

use lazy_static::lazy_static;

lazy_static! {
    static ref INDEX_TEMPLATE: String = include_str!("../../src/templates/index.html").to_string();
}

pub fn render_template(rows: &str) -> String {
    INDEX_TEMPLATE.replace("{ROWS}", rows)
}