pub mod core;
use crossterm::{execute, terminal::{Clear, ClearType}, cursor};
use std::io;

pub trait Describable {
    fn describe(&self) -> String;
    fn describe_field(&self, field: &str) -> String;
}
#[derive(Debug, Clone, Copy)]
pub enum ProductCategory {
    Electronics,
    Clothing,
    Food,
    Books
}
#[derive(Debug, Clone, Copy)]
pub enum StorageType {
    Shelving,
    Refrigerated,
    Bulk
}

pub fn clear_console() {
    execute!(
        io::stdout(),
        Clear(ClearType::All),
        cursor::MoveTo(0,0)
    ).unwrap();
}