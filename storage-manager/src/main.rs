use std::{io};
use storage_manager::core::{Warehouse, warehouse};
use storage_manager::{Describable, clear_console};


fn main() {
    println!("Warehouse manager");
    let mut warehouse_list = Vec::<Warehouse>::new();

    let mut input = String::new();

    'main_prog: loop {
        input.clear();
        clear_console();

        println!("Choose an option:\n 1 - View warehouses \n 2 - Add warehouse \n 3 - Manage warehouse \n 4 - Exit program");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() as &str {
            "1" => {
                for w in warehouse_list.iter() {
                    
                    let output = Describable::describe(w);
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
                    clear_console();

                    let current_warehouse = 'search_loop: loop {
                        input.clear();
                        println!("Enter name of warehouse to manage");

                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");

                        let name_search = input.trim().to_string();

                        let curr_warehouse = warehouse_list
                            .iter_mut()
                            .find(|item| item.name().clone() == name_search);

                        match curr_warehouse {
                            Some(w) => break 'search_loop w,
                            None => {
                                println!("Name not found");
                                continue 'search_loop;
                            }
                        }
                    };
                    'curr_warehouse_loop: loop {
                        clear_console();

                        let mut output = current_warehouse.describe();
                        println!("Chosen warehouse is:\n {output}");

                        input.clear();
                        println!("Choose an option:\n 1 - Manage storage units \n 2 - Manage inventory items \n 3 - exit");

                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");

                        match input.trim() as &str {
                            "1" => {
                                'units_manage_loop: loop {
                                    clear_console();

                                    println!("Managing units \n Choose an option \n 1 - View units \n 2 - Add new unit \n 3 - Exit");
                                    input.clear();

                                    io::stdin()
                                        .read_line(&mut input)
                                        .expect("Failed to read line");

                                    match input.trim() as &str {
                                        "1" => {
                                            for u in current_warehouse.units_list() {
                                                output = Describable::describe(u);

                                                println!("{output}");

                                                let occupied: u64 = current_warehouse
                                                    .inventory()
                                                    .iter()
                                                    .filter(|item| item.placement().id() == u.id())
                                                    .map(|item| item.count())
                                                    .sum();
                                                let free_space = u.capacity() - occupied;

                                                println!("Free space: {free_space}");
                                            }

                                            input.clear();
                                            println!("Enter any key to exit");

                                            io::stdin()
                                                .read_line(&mut input)
                                                .expect("Failed to read line");

                                            if !input.trim().is_empty() {
                                                continue 'units_manage_loop;
                                            }      
                                        },
                                        "2" => {
                                            current_warehouse.add_storage_unit();
                                            continue 'units_manage_loop;
                                        },
                                        "3" => {break 'units_manage_loop},
                                        _ => {
                                            println!("Invalid option. Re-enter");
                                            continue 'units_manage_loop;
                                        }
                                    }
                                }
                                continue 'curr_warehouse_loop;
                            },
                            "2" => {
                                'inventory_manage_loop: loop {
                                    clear_console();

                                    println!("Managing inventory \n Choose an option \n 1 - View items \n 2 - Add new item \n 3 - Exit");
                                    input.clear();

                                    io::stdin()
                                        .read_line(&mut input)
                                        .expect("Failed to read line");

                                    match input.trim() as &str {
                                        "1" => {
                                            for i in current_warehouse.inventory() {
                                                output = Describable::describe(i);

                                                println!("{output}");
                                            }

                                            input.clear();
                                            println!("Enter any key to exit");

                                            io::stdin()
                                                .read_line(&mut input)
                                                .expect("Failed to read line");

                                            if !input.trim().is_empty() {
                                                continue 'inventory_manage_loop;
                                            }                              
                                        },
                                        "2" => {
                                            current_warehouse.add_inventory_item();
                                            continue 'inventory_manage_loop;
                                        },
                                        "3" => {break 'inventory_manage_loop},
                                        _ => {
                                            println!("Invalid option. Re-enter");
                                            continue 'inventory_manage_loop;
                                        }
                                    }
                                }
                                continue 'curr_warehouse_loop;
                            },
                            "3" => {
                                break 'manage_loop;
                            },
                            _ => {
                                println!("Invalid option. Exiting...");
                                continue 'curr_warehouse_loop;
                            }
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
