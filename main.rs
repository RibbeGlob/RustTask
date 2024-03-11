use reqwest::Error;
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::{self, Write};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Deserialize)]
struct ApiResponse {
    conversion_rates: HashMap<String, f64>,
}

#[derive(Deserialize)]
struct ExchangeRateResponse {
    #[serde(rename = "conversion_rate")]
    conversion_rate: f64,
}

static CACHED_DATA: Lazy<Mutex<Option<CachedResponse>>> = Lazy::new(|| Mutex::new(None));

struct CachedResponse {
    response: String,
    expiry: Instant,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("Wybierz opcję:");
        println!("1 - Sprawdź dostępne waluty i ich aktualne kursy");
        println!("2 - Wymiana dwóch podanych walut");

        let option = read_input();

        let result = match option.as_str() {
            "1" => show_available_currencies().await,
            "2" => exchange_currency().await,
            _ => {
                println!("Niepoprawna opcja.");
                continue;
            },
        };

        if let Err(e) = result {
            println!("Error: {}", e);
            continue; // Powrót do menu głównego zamiast kończenia programu
        }

        println!("Czy to wszystko? (tak/nie)");
        let answer = read_input();

        if answer.eq_ignore_ascii_case("tak") {
            break;
        }
    }

    Ok(())
}

async fn fetch_currency_data(word: &str, base_currency: &str, currency2: &str) -> Result<String, Box<dyn std::error::Error>> {
    let cache_key = format!("{}-{}", base_currency, currency2);
    
    {
        let cached_data = CACHED_DATA.lock().unwrap();
        if let Some(cache) = &*cached_data {
            if cache.expiry > Instant::now() && cache_key == cache.response {
                return Ok(cache.response.clone());
            }
        }
    }

    let api_key = "f8c64abf3886e78db302f662"; 
    let url = format!("https://v6.exchangerate-api.com/v6/{}/{}/{}{}", api_key, word, base_currency, currency2);
    let res = reqwest::get(&url).await?;

    match res.status() {
        StatusCode::OK => {
            let body = res.text().await?;
            let mut cached_data = CACHED_DATA.lock().unwrap();
            *cached_data = Some(CachedResponse {
                response: body.clone(),
                expiry: Instant::now() + Duration::from_secs(3600), // Ustawienie czasu wygaśnięcia na 1 godzinę
            });
            Ok(body)
        },
        StatusCode::NOT_FOUND => Err("Podano niepoprawną walutę.".into()),
        StatusCode::TOO_MANY_REQUESTS => Err("Przekroczono limit zapytań do API.".into()),
        _ => Err("Wystąpił nieoczekiwany błąd.".into())
    }
}


async fn show_available_currencies() -> Result<(), Box<dyn std::error::Error>> {

    println!("Enter your base currency (e.g. USD):");
    let base_currency: String = read_input();

    // Wywołaj funkcję fetch_currency_data, aby pobrać dane
    let body = fetch_currency_data("latest", &base_currency,"").await?;

    // Deserializacja treści odpowiedzi do struktury ApiResponse
    let response: ApiResponse = serde_json::from_str(&body)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // Przekonwertowanie HashMap na wektor par (klucz, wartość)
    let mut rates: Vec<(&String, &f64)> = response.conversion_rates.iter().collect();

    // Sortowanie wektora od największego do najmniejszego kursu
    rates.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Wyświetlenie posortowanych kursów walut 
    println!("Kursy walut (od największego do najmniejszego):");
    for (code, rate) in rates {
        println!("{}: {}", code, rate);
    }

    Ok(())
}

async fn exchange_currency() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter your base currency (e.g. USD):");
    let base_currency = read_input();

    println!("Podaj walutę, na którą chcesz wymienić (np. EUR):");
    let target_currency = read_input();

    println!("Podaj ilość wymienianej waluty:");
    let amount: f64 = read_input().parse().expect("Oczekiwano liczby");

    let target = format!("/{}", target_currency);
    let body = fetch_currency_data("pair", &base_currency,&target).await?;

    // Poprawne deserializowanie `String` do `ExchangeRateResponse`
    let response: ExchangeRateResponse = serde_json::from_str(&body)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    let converted_amount = amount * response.conversion_rate;
    println!(
        "Wymieniasz {} {} na {} przy kursie {}, otrzymasz: {:.2} {}",
        amount, base_currency, target_currency, response.conversion_rate, converted_amount, target_currency
    );

    Ok(())
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdout().flush().expect("Błąd przy czyszczeniu bufora");
    io::stdin().read_line(&mut input).expect("Błąd przy odczycie linii");
    input.trim().to_string()
}