use crate::core::Warehouse;
use std::io;

use super::{Product, StorageUnit};

#[derive(Clone)]
pub struct InventoryItem {
    good: Product,
    placement: StorageUnit,
    count: u64,
}

impl InventoryItem {
    pub fn new(good: Product, plac: &StorageUnit, count: u64) -> Self {
        let placement = plac.clone();
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