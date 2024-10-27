use std::io;

fn main() {
    // let num = 200;
    let mut num = String::new();

    println!("Enter a number: ");

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num: i32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            main();
            return;
        },
    };

    println!("Five: {}", five());
    another_function(num);
}

fn another_function(x: i32) {
    println!("the number is {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}