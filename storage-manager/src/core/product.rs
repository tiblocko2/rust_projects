use crate::{Describable, ProductCategory};
use std::io;
use uuid::Uuid;

#[derive(Clone)]
pub struct Product {
    id : Uuid,
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
            "name" => {
                format!("{}", self.name)
            }
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
    pub fn new() -> Self {
        println!("Enter the name of product:");
        
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let name = input.trim().to_string();

        let category = loop {
            println!("Choose a category of the product: \n1 - Electronics \n2 - Clothing \n3 - Food \n4 - Books");
            input.clear();

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

        let cost: f32 = loop {
            println!("Enter a cost of product:");
            input.clear();

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
        let id = Uuid::new_v4();
        Product { 
            id, 
            name, 
            category, 
            cost 
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn id(&self) -> String {
        let id = &self.id.to_string();
        id.to_string()
    }
}
