use crate::{Describable, StorageType};
use std::io;

#[derive(Clone, Copy)]
pub struct StorageUnit {
    id: u64,
    category: StorageType,
    capacity: u64
}

impl Describable for StorageUnit {
    fn describe(&self) -> String {
        format!(
            "Storade unit ID: {}\n Type: {:?} \n Capacity: {}",
            self.id, self.category, self.capacity
        )
    }
    fn describe_field(&self, field: &str) -> String {
        match field {
            "category" => {
                format!(
                    "Storage unit id: {} has type: {:?}",
                    self.id, self.category
                )
            },
            "capacity" => {
                format!(
                    "Storage unit id: {} has capacity: {}",
                    self.id, self.capacity
                )
            },
            _ => format!("Storage unit with id {} doen't has this field {}", self.id, field)
        }
    }
}

impl StorageUnit {
    pub fn new(id: u64) -> Self {
        let mut input = String::new();
        let category = loop {
            input.clear();
            println!("Choose a type of unit: \n1 - Shelving \n2 - Refrigerated \n3 - Bulk");
            io::stdin()
                .read_line(&mut input)
                .expect("Error of reading str");
            match input.trim() {
                "1" => {
                        break StorageType::Shelving
                    },
                "2" => {
                        break StorageType::Refrigerated
                    },
                "3" => {
                        break StorageType::Bulk
                    },
                _ => {
                    println!("Invalid number. Re-enter");
                    continue
                }
            };
        };

        let capacity: u64 = loop {
            input.clear();
            println!("Enter a capacity of unit:");
            io::stdin()
                .read_line(&mut input)
                .expect("Error of reading str");
            match input.trim().parse() {
                Ok(c) => {
                    break c
                },
                Err(_) => {
                    println!("Enter a valid number");
                    continue;
                }
            }
        };

        StorageUnit { 
            id, 
            category, 
            capacity
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn category(&self) -> &StorageType {
        &self.category
    }

    pub fn capacity(&self) -> u64 {
        self.capacity
    }
}