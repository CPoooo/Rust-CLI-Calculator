use std::io;

fn main() {
    loop {
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
            "a" => add(num1, num2),
            "s" => sub(num1.into(), num2.into()),
            "d" => divide(num1.into(), num2.into()),
            "m" => multiply(num1, num2),
            _ => println!("You chose something else? You imbecile!"),
        }

        println!("Would you like to do more math you nerd? y/n");

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let selection = input.trim();
        match selection {
            "n" => {
                println!("I was tired of you anyways...");
                break;
            }
            "y" => {
                println!("Shouldn't you like go outside or something...\n\n");
                continue;
            }
            _ => {
                println!("Well that was NOT an option, you're going again anyways buddy!");
                break;
            }
        }
    }
}

fn add(n1: u32, n2: u32) {
    println!("{n1} + {n2} = {}", n1 + n2);
}
fn sub(n1: i64, n2: i64) {
    println!("{n1} - {n2} = {}", n1 - n2);
}

fn divide(n1: f64, n2: f64) {
    if n2 == 0.0 {
        println!("Cannot divide by zero!");
    } else {
        println!("{n1} / {n2} = {}", n1 / n2);
    }
}

fn multiply(n1: u32, n2: u32) {
    println!("{n1} * {n2} = {}", n1 * n2);
}
