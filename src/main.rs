extern crate reqwest;

mod exchange_rate;
mod converter;

use std::env;
use dotenv_codegen::dotenv;
use exchange_rate::ApiExchangeRateProvider;
use converter::CurrencyConverter;

#[tokio::main]
async fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 4 {
        println!("Invalid input format.");
        println!("Usage: convert <source_currency_code> <target_currency_code> <amount_to_be_converted>");
        println!("Example: convert eur usd 1000");
        return;
    }

    let amount_source: f64 = match arguments[3].parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid amount for currency conversion.");
            return;
        }
    };

    let currency_code_for_source = &arguments[1].to_uppercase();
    let currency_code_for_target = &arguments[2].to_uppercase();

    let api_base_url = dotenv!("EXCHANGERATE_API_BASE_URL");
    let api_key = dotenv!("EXCHANGERATE_API_KEY");


    let exchange_rate_provider = ApiExchangeRateProvider::new(api_base_url, api_key);
    let currency_converter = CurrencyConverter::new(exchange_rate_provider);

    match currency_converter.get_conversion_rate(currency_code_for_source, currency_code_for_target).await {
        Ok(conversion_rate) => {
            println!();
            println!("Converted amount: {:.3} {}", (amount_source * conversion_rate), currency_code_for_target);
            println!("[ Exchange Rate: 1 {} = {} {} ]", currency_code_for_source, conversion_rate, currency_code_for_target);
        },
        Err(err) => eprintln!("Error: {}", err),
    }
}
