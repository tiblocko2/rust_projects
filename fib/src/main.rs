use std::io;

fn main() {
    println!("Calc to Fibonacci number");

    'main_prog: loop {
        let mut input: String = String::new();

        println!("Enter number");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let n: u32 = match input.trim().parse() {
            Ok(num) => {
                if num > 50 {
                    let num = 50;
                    num
                } else {
                    num
                }
            }
            Err(_) => {
                println!("Enter correct int");
                continue 'main_prog;
            }
        };

        let res_i_fib = calc_i_fun(n);

        println!("The result with iterative method: {res_i_fib} with number {n}");

        let res_fib = calc_rec_fun(n);

        println!("The result fib: {res_fib} with number {n}");
    }
}


//рекурсивный обычный
fn calc_rec_fun(n: u32) -> u128 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        calc_rec_fun(n-1) + calc_rec_fun(n-2)
    }
}

//итеративный метод
fn calc_i_fun(n: u32) -> u128 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let mut a: u128 = 0;
        let mut b: u128 = 1;

        for _ in 2..=n {
            let next = a + b;
            a = b;
            b = next;
        }

        b
    }
}
