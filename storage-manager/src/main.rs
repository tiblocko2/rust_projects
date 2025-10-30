use std::io;
use storage_manager::core::{InventoryItem, StorageUnit, Warehouse, Product};
use storage_manager::Describable;

fn main() {
    println!("Warehouse manager");
    let mut warehouse_list = Vec::<Warehouse>::new();

    let mut input = String::new();

    'main_prog: loop {
        input.clear();

        println!("Choose an option:\n 1 - View warehouses \n 2 - Add warehouse \n 3 - Manage warehouse \n 4 - Exit program");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() as &str {
            "1" => {
                for w in warehouse_list.iter().cloned() {
                    
                    let mut output = Describable::describe(&w);
                    println!("{output}");
                }
                continue 'main_prog;
            },
            "2" => {
                let warehouse = Warehouse::new();

                warehouse_list.push(warehouse);

                continue 'main_prog;
            },
            "3" => {
                'manage_loop: loop {
                    input.clear();
                    let mut current_warehouse : Warehouse;
                    println!("Enter name of warehouse to manage");

                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");

                    let name_search = input.trim().to_string();

                    let curr_warehouse = warehouse_list
                        .into_iter()
                        .filter(|item| item.name().clone() == name_search)
                        .nth(0);

                    current_warehouse = curr_warehouse
                        .expect("Name not found")
                        .clone();

                    let mut output = current_warehouse.describe();
                    println!("Chosen warehouse is:\n {output}");

                    input.clear();
                    println!("Choose an option:\n 1 - Add storage unit \n 2 - Add inventory item \n 3 - exit");

                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");

                    match input.trim() as &str {
                        "1" => {
                            current_warehouse.add_storage_unit();
                            warehouse_list.push(current_warehouse);
                            continue 'manage_loop;
                        },
                        "2" => {
                            current_warehouse.add_inventory_item();
                            warehouse_list.push(current_warehouse);
                            continue 'manage_loop;
                        },
                        "3" => {
                            break 'manage_loop;
                        },
                        _ => {
                            println!("Invalid option. Exiting...");
                            continue 'manage_loop;
                        }
                    }
                }

                continue 'main_prog;
            },
            "4" => {
                break 'main_prog;
            },
            _ => {
                println!("Invalid option. Re-enter");
                
                continue 'main_prog;
            }
        }
    }
}
