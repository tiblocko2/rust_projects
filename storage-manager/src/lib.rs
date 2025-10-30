pub mod core;

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
