use super::{Product, StorageUnit};

pub struct InventoryItem {
    good: Product,
    placement: StorageUnit,
    count: u128
}

