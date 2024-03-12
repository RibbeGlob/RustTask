use clap::{App, Arg, ArgMatches};
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Deserialize)]
struct ApiResponse {
    conversion_rates: HashMap<String, f64>,
}

#[derive(Deserialize)]
struct ExchangeRateResponse {
    conversion_rate: f64,
}

static CACHED_DATA: Lazy<Mutex<HashMap<String, CachedResponse>>> = Lazy::new(|| Mutex::new(HashMap::new()));

struct CachedResponse {
    response: String,
    expiry: Instant,
}

// The main function launched when the program starts
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new("Currency Converter")
        .version("1.0")
        .about("Converts amounts between different currencies using real-time exchange rate data.")
        .arg(Arg::with_name("source").long("source").takes_value(true).help("Source currency code"))
        .arg(Arg::with_name("target").long("target").takes_value(true).help("Target currency code"))
        .arg(Arg::with_name("amount").long("amount").takes_value(true).help("Amount to be converted"))
        .arg(Arg::with_name("interactive").long("interactive").help("Activates interactive mode"))
        .arg(Arg::with_name("exrate").long("exrate").help("exchange rate"));
    let matches = app.get_matches();

    // if interactive is chosen
    if matches.is_present("interactive") {
        run_interactive_mode().await?;
    } 

    // if exchange rate is chosen
    else if matches.is_present("exrate") {
        if let Some(source)=matches.value_of("source"){
            show_available_currencies(source).await?;
        }
    }

    // if all data is given
    else {
        process_conversion(&matches).await?;
    }

    Ok(())
}


// Getting the value of data
async fn process_conversion(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let source_currency = matches.value_of("source").unwrap_or_default();
    let target_currency = matches.value_of("target").unwrap_or_default();
    let amount = matches.value_of("amount").unwrap_or("1").parse::<f64>()?; // Default value = 1

    if target_currency.is_empty() {
        show_available_currencies(source_currency).await
    } 
    else {
        exchange_currency(source_currency, target_currency, amount).await
    }
}


// Interactive main menu
async fn run_interactive_mode() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the interactive currency converter!");
    
    loop {
        println!("Select an option:");
        println!("1 - Check available currencies and their current rates");
        println!("2 - Exchange of two given currencies");

        let option = read_input();

        match option.as_str() {
            "1" => {
                println!("Enter your base currency (e.g., USD):");
                let base_currency = read_input();
                if let Err(e) = show_available_currencies(&base_currency).await {
                    println!("Error: {}", e);
                }
            },
            "2" => {
                println!("Enter your base currency (e.g., USD):");
                let base_currency = read_input();
                println!("Enter the target currency (e.g., EUR):");
                let target_currency = read_input();
                println!("Enter the amount to convert:");
                let amount: f64 = read_input().parse().unwrap_or(1.0); // default value 1 if there is error
                if let Err(e) = exchange_currency(&base_currency, &target_currency, amount).await {
                    println!("Error: {}", e);
                }
            },
            _ => println!("Invalid option."),
        };

        println!("Is that all? (yes/no)");
        let answer = read_input();

        if answer.eq_ignore_ascii_case("yes") {
            break;
        }
    }

    Ok(())
}


// Getting value from API
async fn fetch_currency_data(word: &str, base_currency: &str, currency2: &str) -> Result<String, Box<dyn std::error::Error>> {
    let cache_key = format!("{}-{}", base_currency, currency2);

    // Buffer
    {
        let cached_data = CACHED_DATA.lock().unwrap();
        if let Some(cached_response) = cached_data.get(&cache_key) {
            if cached_response.expiry > Instant::now() {
                return Ok(cached_response.response.clone());
            }
        }
    }

    let api_key = "f8c64abf3886e78db302f662"; 
    let url = format!("https://v6.exchangerate-api.com/v6/{}/{}/{}{}", api_key, word, base_currency, currency2);
    let res = reqwest::get(&url).await?;

    // HTTP answer
    match res.status() {
        StatusCode::OK => {
            let body = res.text().await?;
            let mut cache = CACHED_DATA.lock().unwrap();
            cache.insert(cache_key, CachedResponse {
                response: body.clone(),
                expiry: Instant::now() + Duration::from_secs(3600),
            });
            Ok(body)
        },
        StatusCode::NOT_FOUND => Err("Invalid currency code.".into()),
        StatusCode::TOO_MANY_REQUESTS => Err("API request limit exceeded.".into()),
        _ => {
            let error_body = res.text().await.unwrap_or_else(|_| "Failed to read response body.".into());
            Err(format!("An unexpected error occurred: {}", error_body).into())
        },
    }
}


// Current exchange rates
async fn show_available_currencies(base_currency: &str) -> Result<(), Box<dyn std::error::Error>> {
    let body = fetch_currency_data("latest", base_currency, "").await?;
    let response: ApiResponse = serde_json::from_str(&body)?;

    let mut rates: Vec<(&String, &f64)> = response.conversion_rates.iter().collect();
    rates.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));

    println!("Exchange rates for {}:", base_currency);
    for (code, rate) in rates {
        println!("{}: {:.4}", code, rate);
    }

    Ok(())
}


// Exchange of two currencies
async fn exchange_currency(source_currency: &str, target_currency: &str, amount: f64) -> Result<(), Box<dyn std::error::Error>> {
    let body = fetch_currency_data("pair", source_currency, &format!("/{}", target_currency)).await?;
    let response: ExchangeRateResponse = serde_json::from_str(&body)?;

    let converted_amount = amount * response.conversion_rate;
    println!("{} {} = {:.2} {} at an exchange rate of {}", amount, source_currency, converted_amount, target_currency, response.conversion_rate);

    Ok(())
}


// Reading input from keyboard
fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}