use std::fmt;
use reqwest::Error;
use serde::Deserialize;
use async_trait::async_trait;

#[async_trait]
pub trait ExchangeRateProvider {
    async fn get_exchange_rate(&self, from: &str, to: &str) -> Result<f64, ExchangeRateError>;
}

#[derive(Debug)]
pub enum ExchangeRateError {
    RequestError(Error),
    RateNotFound(String, String),
}

impl fmt::Display for ExchangeRateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExchangeRateError::RequestError(err) => write!(f, "Request error: {}", err),
            ExchangeRateError::RateNotFound(from, to) => write!(f, "Exchange rate from {} to {} not found.", from, to)
        }
    }
}

impl From<Error> for ExchangeRateError {
    fn from(error: Error) -> Self {
        ExchangeRateError::RequestError(error)
    }
}

pub struct ApiExchangeRateProvider {
    pub api_base_url: String,
    pub api_key: String,
    pub client: &'static reqwest::Client,
}

impl ApiExchangeRateProvider {
    pub fn new(api_base_url: &str, api_key: &str) -> Self {
        lazy_static::lazy_static! {
            static ref CLIENT: reqwest::Client = reqwest::Client::new();
        }

        ApiExchangeRateProvider {
            api_base_url: api_base_url.to_string(),
            api_key: api_key.to_string(),
            client: &CLIENT,
        }
    }
}

#[derive(Deserialize)]
struct ExchangeRates {
    conversion_rates: std::collections::HashMap<String, f64>,
}

#[async_trait]
impl ExchangeRateProvider for ApiExchangeRateProvider {
    async fn get_exchange_rate(&self, from: &str, to: &str) -> Result<f64, ExchangeRateError> {
        let url = format!("{}/{}/latest/{}", self.api_base_url, self.api_key, from);
        let response = self.client.get(&url).send().await?;

        let exchange_rates: ExchangeRates = response.json().await?;
        match exchange_rates.conversion_rates.get(to) {
            Some(rate) => Ok(*rate),
            None => Err(ExchangeRateError::RateNotFound(from.to_string(), to.to_string()))
        }
    }
}