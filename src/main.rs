use rand::seq::SliceRandom;
use rand::thread_rng;

/// A simple weather forecast demo program that showcases Rust development environment setup.
///
/// This program demonstrates:
/// - Using external crates (rand)
/// - Basic Rust syntax and concepts
/// - Random selection from arrays
/// - String formatting and printing
fn get_random_forecast<'a>(forecasts: &'a [&str]) -> Option<&'a str> {
    let mut rng = thread_rng();
    forecasts.choose(&mut rng).copied()
}

fn main() {
    let forecasts = [
        "sunny", "cloudy", "rainy", "stormy", "windy", "foggy", "snowy", "humid", "clear",
        "drizzly",
    ];
    let forecast = get_random_forecast(&forecasts).unwrap();
    println!("Hello from the Rust development environment!");
    println!("Today's weather forecast: {}", forecast);
    println!("This demo shows that your Rust environment is working correctly! 🦀");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_forecast_returns_valid_forecast() {
        let forecasts = [
            "sunny", "cloudy", "rainy", "stormy", "windy", "foggy", "snowy", "humid", "clear",
            "drizzly",
        ];
        let forecast = get_random_forecast(&forecasts);
        assert!(forecast.is_some());
        assert!(forecasts.contains(&forecast.unwrap()));
    }

    #[test]
    fn test_get_random_forecast_empty_array() {
        let forecasts: [&str; 0] = [];
        let forecast = get_random_forecast(&forecasts);
        assert!(forecast.is_none());
    }
}
