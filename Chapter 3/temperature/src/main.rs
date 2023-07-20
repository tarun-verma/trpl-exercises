use std::io;

fn fa_to_cel(temperature: f32) -> f32 {
    return ((temperature * 5.0) - 160.0) / 9.0;
}

fn cel_to_fa(temperature: f32) -> f32 {
    return ((9.0 * temperature) / 5.0) + 32.0;
}

fn main() {
    let mut temperature = String::new();
    let mut c_or_f = String::new();

    let mut final_temp: f32 = 0.0;

    println!("Please enter the temperature: ");
    io::stdin().read_line(&mut temperature).expect("Failed to read line!");

    println!("Is this in Fahrenheit or Celsius? Press F/C accordingly: ");
    io::stdin().read_line(&mut c_or_f).expect("Failed to read line!");

    let final_unit = c_or_f.trim();

    let trimmed = temperature.trim();
    match trimmed.parse::<f32>() {
        Ok(i) => final_temp = i,
        Err(..) => println!("this was not a float: {}", trimmed),
    };

    if final_unit == "F" {
        println!("The corresponding temperature in Celsius is: {}", fa_to_cel(final_temp));
    }

    if final_unit == "C" {
        println!("The corresponding temperature in Fahrenheit is: {}", cel_to_fa(final_temp));
    }
}
