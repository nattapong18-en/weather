mod weather;
use weather::fetch;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let args: Vec<String> = std::env::args().collect();
    let city = &args[1];
    let weather = fetch(city)?;
    let condition = &weather.current_condition[0];
    let desc = &condition.weather_desc[0].value;

    println!("🌤 {}", desc);
    println!("🌡 {}°C", condition.temp_c);
    println!("💧 {}%", condition.humidity);
   
    Ok(())
}
