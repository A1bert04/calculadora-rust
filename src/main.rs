use std::io;

fn main() {
    println!("Welcome to SplushGay Max Calculator 69 supreme");
    for _ in 0..30 {
        print!("-");
    }

    loop {
        print_menu();
        let option = get_input();

        println!("You selected option number {}", option);

        if option == 5 {
            exit();
        }

        let (a, b) = get_two_numbers();

        match option {
            1 => add(a, b),
            2 => subtract(a, b),
            3 => multiply(a, b),
            4 => divide(a, b),
            _ => println!("Invalid option"),
        }
    }
}

fn print_menu() {
    println!("\nPlease select an option:");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    println!("5. Exit");
}

fn get_input() -> u8 {
    let mut input = String::new();

    loop {
        println!("Please enter a valid number:");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if let Ok(number) = input.trim().parse::<u8>() {
            if number <= 5 {
                return number;
            }
        }

        input.clear();
    }
}

fn add(a: f64, b: f64) {
    println!("{} + {} = {}", a, b, a + b);
}

fn subtract(a: f64, b: f64) {
    println!("{} - {} = {}", a, b, a - b);
}

fn multiply(a: f64, b: f64) {
    println!("{} * {} = {}", a, b, a * b);
}

fn divide(a: f64, b: f64) {
    if b != 0.0 {
        println!("{} / {} = {}", a, b, a / b);
    } else {
        println!("Division by zero is not allowed");
    }
}

fn exit() {
    println!("Goodbye!");
    std::process::exit(0);
}

fn get_two_numbers() -> (f64, f64) {
    let mut a = String::new();
    let mut b = String::new();

    let mut right = false;

    while !right {
        println!("Please enter the first number:");
        io::stdin().read_line(&mut a).expect("Failed to read line");

        if let Ok(num) = a.trim().parse::<f64>() {
            right = true;
            a = num.to_string();
        } else {
            a.clear();
        }
    }

    right = false;

    while !right {
        println!("Please enter the second number:");
        io::stdin().read_line(&mut b).expect("Failed to read line");

        if let Ok(num) = b.trim().parse::<f64>() {
            right = true;
            b = num.to_string();
        } else {
            b.clear();
        }
    }

    (a.trim().parse().unwrap(), b.trim().parse().unwrap())
}
