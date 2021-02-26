use std::io;

fn main() {
    loop {
        println!("選取功能：");
        println!("1. 華氏轉攝氏");
        println!("2. 攝氏轉華氏");
        let mut function_type = String::new();
        io::stdin()
            .read_line(&mut function_type)
            .expect("Failed to read line");

        let temperature_convert_function: fn(f64) -> f64;

        match function_type.trim().as_ref() {
            "1" => {
                temperature_convert_function = fahrenheit_to_celsius;
            }
            "2" => {
                temperature_convert_function = celsius_to_fahrenheit;
            }
            _ => {
                println!("End!");
                return;
            }
        }

        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(t) => t,
            Err(_) => continue,
        };
        let result = temperature_convert_function(temperature);
        println!("{} -> {}", temperature, result);
    }
}

fn fahrenheit_to_celsius(t: f64) -> f64 {
    (t - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(t: f64) -> f64 {
    (t * 9.0 / 5.0) + 32.0
}
