use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
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

    let rng = thread_rng();
    let forecast = forecasts.choose(&mut rng).unwrap();

    println!("Hello!");
    println!("Today's weather forecast: {}", forecast);
}