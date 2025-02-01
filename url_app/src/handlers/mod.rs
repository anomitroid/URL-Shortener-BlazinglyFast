pub mod frontend;
pub mod api;
pub mod redirect;
pub mod static_files;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref INDEX_HTML: &'static str = include_str!("../../src/templates/index.html");
}