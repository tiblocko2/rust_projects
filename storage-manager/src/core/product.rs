use crate::{ProductCategory, Describable};
use std::io;

pub struct Product {
    id : u64,
    name: String,
    category: ProductCategory,
    cost: f32,
}

impl Describable for Product {
    fn describe(&self) -> String {
        format!(
            "Product '{}' \n ID: {} \n Category: {:?} \n Cost: {}",
            self.name, self.id, self.category, self.cost
        )
    }
    fn describe_field(&self, field: &str) -> String {
        match field {
            "id" => {
                format!(
                    "Product: {} has ID: {}",
                    self.name, self.id
                )
            },
            "category" => {
                format!(
                    "Product: {} has Category: {:?}",
                    self.name, self.category
                )
            },
            "cost" => {
                format!(
                    "Product: {} has Cost: {}",
                    self.name, self.cost
                )
            },
            _ => format!("Product {} doen't has this field {}", self.name, field)
        }
    }
}

impl Product {
    pub fn new(id: u64) -> Self {
        println!("Enter the name of product:");
        
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let name = input.trim().to_string();

        let category = loop {
            println!("Choose a category of the product: \n1 - Electronics \n2 - Clothing \n3 - Food \n4 - Books");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            match input.trim() {
                "1" => {break ProductCategory::Electronics},
                "2" => {break ProductCategory::Clothing},
                "3" => {break ProductCategory::Food},
                "4" => {break ProductCategory::Books},
                _ => {
                    println!("Invalid number. Re-enter");
                    continue;
                }
            }
        };

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let cost: f32 = loop {
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

        Product { 
            id, 
            name, 
            category, 
            cost 
        }
    }
}
