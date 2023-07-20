use std::io;

fn fibonacci(index: u32) -> u32 {
    if index == 1 {
        return 1;
    }
    else if index == 2 {
        return 1;
    }
    else {
        return fibonacci(index - 1) + fibonacci(index - 2);
    }
}
fn main() {
    let mut input = String::new();

    println!("Please enter the fibonacci index: ");
    io::stdin().read_line(&mut input).expect("Failed to read line!");

    let input_trimmed = input.trim();

    let mut index: u32 = 0;

    match input_trimmed.parse::<u32>() {
        Ok(i) => index = i,
        Err(..) => println!("This is not a valid integer value: {}", input_trimmed),
    };

    println!("The corresponding fibonacci number is: {}", fibonacci(index));
}
