use chrono::NaiveDate; // <-- Nueva importación
use clap::Parser;
use serde::Deserialize;
use std::process;

// --- DEFINICIÓN DE COMANDOS (CLAP) ---
#[derive(Parser, Debug)]
#[command(name = "weather", about = "Minimalist Weather Forecast")]
struct Args {
    #[arg(short, long)]
    city: String,

    #[arg(long, default_value_t = false)]
    today: bool,

    #[arg(long, default_value_t = false)]
    week: bool,
}

// --- ESTRUCTURAS PARA EL JSON ---
#[derive(Deserialize, Debug)]
struct GeoResponse {
    results: Option<Vec<CityData>>,
}

#[derive(Deserialize, Debug)]
struct CityData {
    latitude: f64,
    longitude: f64,
    name: String,
    country: String,
}

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    current: Option<CurrentWeather>,
    daily: Option<DailyWeather>,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temperature_2m: f32,
    precipitation: f32,
}

#[derive(Deserialize, Debug)]
struct DailyWeather {
    time: Vec<String>,
    temperature_2m_max: Vec<f32>,
    temperature_2m_min: Vec<f32>,
    precipitation_sum: Vec<f32>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // 1. Obtener coordenadas
    let city_info = match get_coordinates(&args.city).await {
        Ok(info) => info,
        Err(_) => {
            eprintln!("Error: Could not find city '{}'", args.city);
            process::exit(1);
        }
    };

    println!(
        "\n  FORECAST: {} ({})",
        city_info.name.to_uppercase(),
        city_info.country
    );
    println!("  {}\n", "-".repeat(50));

    // 2. Obtener datos del clima
    match get_weather(city_info.latitude, city_info.longitude, args.week).await {
        Ok(weather) => {
            if args.week {
                if let Some(daily) = weather.daily {
                    println!("  {:14} | MAX     | MIN     | RAIN", "DATE");
                    println!("  {:-<50}", "");

                    for i in 0..daily.time.len() {
                        let raw_date = &daily.time[i];

                        let formatted_date = match NaiveDate::parse_from_str(raw_date, "%Y-%m-%d") {
                            Ok(parsed) => parsed.format("%A(%d)").to_string(), // %A = Nombre del día, %d = Número
                            Err(_) => raw_date.to_string(), // Si falla, imprimimos el texto original
                        };

                        let t_max = daily.temperature_2m_max[i];
                        let t_min = daily.temperature_2m_min[i];
                        let rain = daily.precipitation_sum[i];

                        // {:14} asegura que todas las fechas ocupen exactamente 14 espacios para no romper la tabla
                        println!(
                            "  {:14} | {:5.1}°C | {:5.1}°C | {:5.1}mm",
                            formatted_date, t_max, t_min, rain
                        );
                    }
                }
            } else {
                if let Some(current) = weather.current {
                    println!("  [ RIGHT NOW ]");
                    println!("  Temp: {:.1} °C", current.temperature_2m);
                    println!("  Rain: {:.1} mm", current.precipitation);
                }
            }
        }
        Err(_) => {
            eprintln!("  [!] Could not fetch weather data.");
        }
    }
    println!();
}

// --- FUNCIONES DE API ---

async fn get_coordinates(city: &str) -> Result<CityData, Box<dyn std::error::Error>> {
    let url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=en&format=json",
        city
    );
    let resp = reqwest::get(url).await?.json::<GeoResponse>().await?;

    if let Some(results) = resp.results {
        if let Some(first) = results.into_iter().next() {
            return Ok(first);
        }
    }
    Err("No city found".into())
}

async fn get_weather(
    lat: f64,
    lon: f64,
    is_week: bool,
) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
    let url = if is_week {
        format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&daily=temperature_2m_max,temperature_2m_min,precipitation_sum&timezone=auto",
            lat, lon
        )
    } else {
        format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,precipitation&timezone=auto",
            lat, lon
        )
    };

    let resp = reqwest::get(url).await?.json::<WeatherResponse>().await?;
    Ok(resp)
}
