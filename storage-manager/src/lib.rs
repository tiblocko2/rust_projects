pub mod core;

trait Describable {
    fn describe(&self) -> String;
    fn describe_field(&self, field: &str) -> String;
}
#[derive(Debug)]
pub enum ProductCategory {
    Electronics,
    Clothing,
    Food,
    Books
}
#[derive(Debug)]
pub enum StorageType {
    Shelving,
    Refrigerated,
    Bulk
}
