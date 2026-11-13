use std::io;

const C: f32 = 32.0;

fn c_to_f(celsius_temp: f32) -> f32 {
    (celsius_temp * (9.0/5.0)) + C
}

fn f_to_c(fahrenheit_temp: f32) -> f32 {
    (fahrenheit_temp - C) * (5.0/9.0)
}

fn convert(temperature: f32, choice: u8) -> Option<f32> {
    match choice {
        1 => Some(c_to_f(temperature)),
        2 => Some(f_to_c(temperature)),
        _ => None,        
    }
}

fn main() {
    println!("Temperature cpnverter.\n (1) C to F\n (2) F to C");
    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice).unwrap();

    let n_choice = user_choice
        .trim()
        .parse::<u8>()
        .expect("Error");

    println!("Enter temperature");

    let mut temerature = String::new();
    io::stdin().read_line(&mut temerature).unwrap();
    
    let temperature = temerature
        .trim()
        .parse::<f32>()
        .expect("Error");

    match convert(temperature, n_choice) {
        Some(result) => println!("The result is: {result}"),
        None => println!("Huy")
    };

}
