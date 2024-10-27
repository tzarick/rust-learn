use std::io;

fn main() {
    looper();

    println!("0C is equal to {}F", temp_conversion(false, 0.0));
    println!("32C is equal to {}F", temp_conversion(false, 32.0));

    println!("32F is equal to {}C", temp_conversion(true, 32.0));
    println!("0F is equal to {}C", temp_conversion(true, 0.0));

    println!("100C is equal to {}F", temp_conversion(false, 100.0));

    println!("------------------------------");

    println!("gen_nth_fib(5) is equal to {}", gen_nth_fib(5));

    println!("------------------------------");
    println!("Hello, world!");

    let mut input = String::new();

    println!("Enter \"even\" or \"odd\"");
    io::stdin().read_line(&mut input).expect("Failed to read line");


    
    if input.trim() == "even" {
        branch(true);
    } else if input.trim() == "odd" {
        branch(false);
    } else {
        println!("Invalid input. input \"even\" or \"odd\"");
    }
}

fn branch(even: bool) {
    let num = if even { 200 } else { 201 };

    if num % 2 == 0 {
        println!("even");
    } else {
        println!("odd");
    }
}

fn looper() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result: {}", result);
}

fn temp_conversion(f_to_c: bool, temp: f64) -> f64 {
    if f_to_c {
        // C = (F - 32) * 5/9
        (temp - 32.0) * 5.0 / 9.0
    } else {
        // F = (C * 9/5) + 32
        (temp * 9.0 / 5.0) + 32.0
    }
}

fn gen_nth_fib(n: u32) -> u32 {
    if n < 1 {
        panic!("n must be greater than 0");
    }
    if n == 1 || n == 2 {
        n - 1
    } else {
        gen_nth_fib(n - 1) + gen_nth_fib(n - 2)
    }
}