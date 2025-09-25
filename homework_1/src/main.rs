// Assignment 1: Temperature Converter
// =======================
const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}


fn temperature_converter() {
    println!("--- Assignment 1: Temperature Converter ---");
    let mut temp_f: f64 = 32.0;
    println!("{}°F = {:.2}°C", temp_f, fahrenheit_to_celsius(temp_f));

    for _i in 1..=5 {
        temp_f += 1.0;
        println!("{}°F = {:.2}°C", temp_f, fahrenheit_to_celsius(temp_f));
    }
    println!();
}

// Assignment 2: Number Analyzer
// =======================
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn number_analyzer() {
    println!("--- Assignment 2: Number Analyzer ---");
    let numbers: [i32; 10] = [3, 5, 10, 15, 7, 8, 12, 20, 25, 30];

    for &num in numbers.iter() {
        if num % 15 == 0 {
            println!("{} -> FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{} -> Fizz", num);
        } else if num % 5 == 0 {
            println!("{} -> Buzz", num);
        } else if is_even(num) {
            println!("{} -> Even", num);
        } else {
            println!("{} -> Odd", num);
        }
    }

    // Sum using while loop
    let mut i = 0;
    let mut sum = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of numbers = {}", sum);

    // Largest number using loop
    let mut largest = numbers[0];
    let mut j = 1;
    loop {
        if j >= numbers.len() {
            break;
        }
        if numbers[j] > largest {
            largest = numbers[j];
        }
        j += 1;
    }
    println!("Largest number = {}", largest);
    println!();
}

// Assignment 3: Guessing Game
// =======================
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn guessing_game() {
    println!("--- Assignment 3: Guessing Game ---");
    let secret: i32 = 7; // secret number
    let mut attempts = 0;

    // Simulated guesses
    let guesses = [3, 5, 9, 7];
    
    loop {
        let guess = guesses[attempts];
        attempts += 1;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Guess {}: {} is correct! 🎉", attempts, guess);
            break;
        } else if result == 1 {
            println!("Guess {}: {} is too high!", attempts, guess);
        } else {
            println!("Guess {}: {} is too low!", attempts, guess);
        }
    }

    println!("It took {} guesses to find the secret number.", attempts);
    println!();
}

// Main Function
// =======================
fn main() {
    temperature_converter();
    number_analyzer();
    guessing_game();
}
