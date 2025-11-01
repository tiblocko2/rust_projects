use std::io;

fn main() {
    println!("C to F converter!");

    'main_prog: loop  {
        println!("Choose an option:");
        println!("1 - Convert to Celsius");
        println!("2 - Convert to Fahrenheit");
        println!("3 - Exit");

        let mut opt: String = String::new();

        io::stdin()
            .read_line(&mut opt)
            .expect("Failed to read line");

        println!("Your chose: {opt}");

        let opt: u32 = match opt.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter correct int");
                continue 'main_prog;
            }
        };

        match opt {
            1 => conv_to_celc(),
            2 => conv_to_fahr(),
            3 => break 'main_prog,
            _ => {
                println!("Enter correct int");
                continue 'main_prog;
            }
        };
    }
}

fn conv_to_celc() {
    println!("Enter the temperature in Fahrenheit");

    let mut input = String::new();

    loop {
        input.clear(); // Очищаем строку перед каждым вводом

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let fahrenheit: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        let celsius = (fahrenheit - 32.0) / 1.8;

        println!("Temperature in Celsius: {:.2}", celsius);
        break;
    }
}

fn conv_to_fahr() {
    println!("Enter the temperature in Celsius");

    let mut input = String::new();

    loop {
        input.clear(); // Очищаем строку перед каждым вводом

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let celsius: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        let fahrenheit = (celsius * 1.8) + 32.0;

        println!("Temperature in Celsius: {:.2}", fahrenheit);
        break;
    }
}