use crate::Describable;

use super::{Product, StorageUnit};

#[derive(Clone)]
pub struct InventoryItem {
    good: Product,
    placement: StorageUnit,
    count: u64,
}



impl InventoryItem {
    pub fn new(good: Product, plac: &StorageUnit, count: u64) -> Self {
        let placement = *plac;
        InventoryItem { 
            good, 
            placement, 
            count 
        }
    }

    pub fn good(&self) -> &Product {
        &self.good
    }

    pub fn placement(&self) -> &StorageUnit {
        &self.placement
    }

    pub fn count(&self) -> u64 {
        self.count
    }
}

impl Describable for InventoryItem {
    fn describe(&self) -> String {
        format!("{} in unit with ID = {} \n Count = {}", self.good.name(), self.placement.id(), self.count)
    }
    fn describe_field(&self, field: &str) -> String {
        format!("Empty trait function")
    }
}
