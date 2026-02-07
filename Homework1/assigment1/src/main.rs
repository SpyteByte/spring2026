//Constant for the freezing point
const FREEZING_POINT_F: f64 = 32.0;

// Converts Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

// Converts Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT_F
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

//Function to check the guess against the secret number
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let mut temp_f: f64 = FREEZING_POINT_F;

    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{temp_f}°F = {:.2}°C", temp_c);

    for _ in 0..5 {
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celsius(temp_f);
        println!("{temp_f}°F = {:.2}°C", temp_c);
    }
//Array of 10 integers
    let numbers: [i32; 10] = [3, 7, 10, 15, 22, 30, 41, 50, 63, 88];

    // Even or Odd
    for &num in numbers.iter() {
        if is_even(num) {
            print!("{num} is even → ");
        } else {
            print!("{num} is odd → ");
        }

        // FizzBuzz
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else {
            println!("no FizzBuzz");
        }
    }

    let mut index = 0;
    let mut sum = 0;

    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }

    println!("\nSum of all numbers: {sum}");

    //find the largest number
    let mut largest = numbers[0];

    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }

    println!("Largest number in the array: {largest}");
//Secret number
    let secret: i32 = 15;

    // Mutable guess variable
    let mut guess: i32 = 1;

    // Counter
    let mut attempts: i32 = 0;

    loop {
        attempts += 1;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Guess {guess} is correct!");
            break;
        } else if result == 1 {
            println!("Guess {guess} is too high.");
        } else {
            println!("Guess {guess} is too low.");
        }

        guess += 1;
    }

    // Print total number of guesses
    println!("It took {attempts} guesses to find the secret number.");
    
}


