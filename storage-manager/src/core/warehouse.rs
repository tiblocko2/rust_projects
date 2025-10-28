use super::{InventoryItem, StorageUnit};
use crate::{Describable};
use std::io;

pub struct Warehouse {
    name: String,
    units_list: Vec<StorageUnit>,
    inventory: Vec<InventoryItem>,
    next_id_unit: u64,
}

impl Describable for Warehouse {
    fn describe(&self) -> String {
        format!("Name: {} \nStorage units: {}\nInventory: {}", self.name, self.units_list.len(), self.inventory.len())
    }
    fn describe_field(&self, field: &str) -> String {
        match field {
            "name" => {
                format!(
                    "Warehouse name: {}",
                    self.name
                )
            },
            "units" => {
                format!(
                    "Warehouse has count of units: {}",
                    self.units_list.len()
                )
            },
            "inventory" => {
                format!(
                    "Warehouse has count of inventory: {}",
                    self.inventory.len()
                )
            },
            _ => format!("Warehouse {} doen't has this field {}", self.name, field)
        }
    }
}

impl Warehouse {
    pub fn new() -> Self {
        println!("Enter warehouse name: ");
        let mut name = String::new();


        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        let name = name.trim().to_string();

        Warehouse { 
            name, 
            units_list: Vec::new(), 
            inventory: Vec::new(),
            next_id_unit: 1,
        }
    }
    pub fn add_storage_unit(&mut self) {
        let id = self.next_id_unit;

        let unit: StorageUnit = StorageUnit::new(id);

        self.next_id_unit += 1;
        self.units_list.push(unit);
    }
    fn add_product_to_inventory_by_id(&mut self) {

    }
}