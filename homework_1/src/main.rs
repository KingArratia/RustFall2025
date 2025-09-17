const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT_F
}

fn assignment1() {
    // Temp converter
    let mut f = 32.0;
    for _ in 0..6 {
        println!("{:.1}°F = {:.1}°C", f, fahrenheit_to_celsius(f));
        f += 1.0;
    }
}

fn main() {
    assignment1();
}
