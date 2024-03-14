use crate::exchange_rate::ExchangeRateProvider;

pub struct CurrencyConverter<T: ExchangeRateProvider> {
    pub conversion_rate_provider: T,
}

impl<T: ExchangeRateProvider> CurrencyConverter<T> {
    pub fn new(provider: T) -> Self {
        CurrencyConverter {
            conversion_rate_provider: provider,
        }
    }

    pub async fn get_conversion_rate(&self, from: &str, to: &str) -> Result<f64, String> {
        let conversion_rate_result = self.conversion_rate_provider.get_exchange_rate(from, to).await;
        let conversion_rate = match conversion_rate_result {
            Ok(rate) => rate,
            Err(err) => return Err(format!("Error getting exchange rate: {}", err))
        };
        Ok(conversion_rate)
    }
}