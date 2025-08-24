use rand::thread_rng;
use rand::seq::SliceRandom;

/// A simple weather forecast demo program that showcases Rust development environment setup.
/// 
/// This program demonstrates:
/// - Using external crates (rand)
/// - Basic Rust syntax and concepts
/// - Random selection from arrays
/// - String formatting and printing
fn main() {
    // Array of possible weather forecasts
    let forecasts = [
        "sunny",
        "cloudy", 
        "rainy",
        "stormy",
        "windy",
        "foggy",
        "snowy",
        "humid",
        "clear",
        "drizzly",
    ];

    // Create a random number generator
    let mut rng = thread_rng();
    
    // Randomly select a forecast from the array
    let forecast = forecasts.choose(&mut rng).unwrap();

    // Display the greeting and weather forecast
    println!("Hello from the Rust development environment!");
    println!("Today's weather forecast: {}", forecast);
    println!("This demo shows that your Rust environment is working correctly! 🦀");
}