use super::{InventoryItem, StorageUnit};
use crate::{Describable, clear_console, core::Product};
use std::io;


#[derive(Clone)]
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

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn units_list(&self) -> &Vec<StorageUnit> {
        &self.units_list
    }

    pub fn inventory(&self) -> &Vec<InventoryItem> {
        &self.inventory
    }

    pub fn add_storage_unit(&mut self) {
        let id = self.next_id_unit;

        let unit: StorageUnit = StorageUnit::new(id);

        self.next_id_unit += 1;
        self.units_list.push(unit);
    }
    
    pub fn add_inventory_item(&mut self) {
        let mut input = String::new();
        let name = self.name();
        println!("New item in {name}");

        let good = Product::new();

        'main: loop {
            println!("Choose an option:\n 1 - Create new storage unit \n 2 - Use existent");
            input.clear();

            io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

            match input.trim() {
                "1" => {
                    self.add_storage_unit();


                    let count = 'count_correct: loop {
                            println!("Enter count of products");
                            input.clear();

                            io::stdin()
                                .read_line(&mut input)
                                .expect("Failed to read line");

                            match input.trim().parse() {
                                Ok(o) => {
                                    break 'count_correct o;
                                },
                                Err(_) => {
                                    println!("Enter a valid number");
                                    continue 'count_correct;
                            }
                        }
                    };

                    for u in &self.units_list {
                        let occupied: u64 = self
                            .inventory
                            .iter()
                            .filter(|item| item.placement().id() == u.id())
                            .map(|item| item.count())
                            .sum();

                        if occupied + count > u.capacity(){
                            println!("Not enough capacity. Choose another unit or count");
                            continue 'main;
                        } else {
                            self.inventory.push(InventoryItem::new(good, &self.units_list[(self.next_id_unit - 2) as usize], count));
                            break 'main;
                        }
                    }


                },
                "2" => {
                    'id_loop: loop {
                        clear_console();

                        println!("Existent units:");
                        for u in &self.units_list {
                            let output = Describable::describe(u);

                            println!("{output}");

                            let occupied: u64 = self
                                .inventory()
                                .into_iter()
                                .filter(|item| item.placement().id() == u.id())
                                .map(|item| item.count())
                                .sum();
                            let free_space = u.capacity() - occupied;

                            println!("Free space: {free_space}");
                        }

                        println!("Enter ID of unit");
                        input.clear();

                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");

                        let id = match input.trim().parse::<u64>() {
                            Ok(id) => {id},
                            Err(_) => {
                                println!("Enter a valid number");

                                continue 'id_loop;
                            }
                        };

                        let unit =match self.units_list.iter().find(|u| u.id() == id) {
                            Some(unit) => {unit},
                            None => {
                                println!("Unit with this ID doesn't exist. Re-enter");
                                continue 'id_loop;
                            }
                        };

                        let count = 'count_correct: loop {
                            println!("Enter count of products");
                            input.clear();

                            io::stdin()
                                .read_line(&mut input)
                                .expect("Failed to read line");

                            match input.trim().parse() {
                                Ok(o) => {
                                    break 'count_correct o;
                                },
                                Err(_) => {
                                    println!("Enter a valid number");
                                    continue 'count_correct;
                                }
                            }
                        };

                        let occupied: u64 = self
                            .inventory()
                            .into_iter()
                            .filter(|item| item.placement().id() == unit.id())
                            .map(|item| item.count())
                            .sum();

                        if occupied + count > unit.capacity(){
                            println!("Not enough capacity. Choose another unit or count");
                            input.clear();
                            println!("Enter any key to exit");

                            io::stdin()
                                .read_line(&mut input)
                                .expect("Failed to read line");

                            if !input.trim().is_empty() {
                                continue 'id_loop;
                            }      
                            
                        } else {
                            self.inventory.push(InventoryItem::new(good, unit, count));
                            break 'id_loop;
                        }                        
                    }

                    break 'main;
                },
                _ => {
                    println!("Invalid option");
                    continue 'main;
                }

            }
        }
    }

    pub fn find_product_by_name(&self) {
        let mut input = String::new();

        println!("Enter name of product:");
        input.clear();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("Found products:");
        for p in self
            .inventory
            .iter()
            .filter(|item| item.good().name().clone() == input.trim().to_string()) {

            let mut output = p.describe();

            println!("{output}");

            output = p.good().describe();

            println!("{output}");
        }
    }

    pub fn remove_product(&mut self) {
        let mut count: u32 = 0;
        for g in self.inventory.iter() {
            let output = g.good().describe_field("name");

            count += 1;

            println!("{count} : {output}");
        }

        let number: u32 = 'num_prod: loop{
            println!("Enter empty to exit \n Enter number of product to remove:");
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if !input.to_string().is_empty() {
                match input.trim().parse() {
                    Ok(num) => {
                        break 'num_prod num;
                    },
                    Err(_) => {
                        println!("Invalid number. Re-enter");
                        continue 'num_prod;
                    }
                }
            } else {
                break 'num_prod 0;
            }
        };

        if number > 0 {
            let item = &self.inventory()[(number - 1) as usize];

            let output = item.describe();

            println!("You are going to remove \n {output} \n y/n? (n by default)");
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match  input.trim() as &str{
                "y" => {
                    let removed = self.inventory.remove((number - 1) as usize);
                    println!("Item {} removed", removed.good().name());
                },
                _ => {
                    println!("Nothing has been removed");
                }
            }
        } else {
            println!("Nothing has been removed");
        }
    }
}