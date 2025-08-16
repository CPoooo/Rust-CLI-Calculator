use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter in two numbers with space between them: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let both_nums: Vec<&str> = input.trim().split(" ").collect();
    let num1: u32 = both_nums[0].parse().expect("not a number");
    let num2: u32 = both_nums[1].parse().expect("not a number");

    println!(
        "Would you like to add, sub, divide, or multiply?\nEnter a (add), s (sub), d (divide), m (multiply)"
    );

    input.clear();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let selection = input.trim();

    match selection {
        "a" => println!("You chose addition"),
        "s" => println!("You chose subtraction"),
        "d" => println!("You chose division"),
        "m" => println!("You chose mutliplication"),
        _ => println!("You chose something else? You imbecile!"),
    }
}

fn add(n1: u32, n2: u32) {
    println!("{n1} + {n2} = {}", n1 + n2);
}
fn sub(n1: u32, n2: u32) {
    println!("{n1} - {n2} = {}", n1 + n2);
}

fn divide(n1: u32, n2: u32) {
    println!("{n1} / {n2} = {}", n1 + n2);
}

fn multiply(n1: u32, n2: u32) {
    println!("{n1} * {n2} = {}", n1 + n2);
}
