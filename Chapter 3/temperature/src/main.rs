fn fa_to_cel(temperature: f32) -> f32 {
    return ((temperature * 5.0) - 160.0) / 9.0;
}

fn cel_to_fa(temperature: f32) -> f32 {
    return ((9.0 * temperature) / 5.0) + 32.0;
}

fn main() {
    println!("33C to F is: {}", cel_to_fa(33.0));
    println!("103F to C is: {}", fa_to_cel(103.0));
}
