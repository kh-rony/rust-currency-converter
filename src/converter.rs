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


#[cfg(test)]
mod tests {
    use crate::converter::CurrencyConverter;
    use crate::exchange_rate::ApiExchangeRateProvider;

    #[tokio::test]
    fn test_invalid_input() {
        let currency_converter = CurrencyConverter::new(ApiExchangeRateProvider::new("https://v6.exchangerate-api.com/v6", "108d1196ef2513db69e7a424"));
        let convertion_rate = currency_converter.get_conversion_rate("INVALID", "EUR");
        assert_eq!(convertion_rate, 0.0);
    }

    #[tokio::test]
    fn test_unsupported_currencies() {
        let currency_converter = CurrencyConverter::new(ApiExchangeRateProvider::new("https://v6.exchangerate-api.com/v6", "108d1196ef2513db69e7a424"));
        let convertion_rate = currency_converter.get_conversion_rate("EUR", "UNSUPPORTED");
        assert_eq!(convertion_rate, 0.0);
    }

    #[tokio::test]
    fn test_edge_cases() {
        let currency_converter = CurrencyConverter::new(ApiExchangeRateProvider::new("https://v6.exchangerate-api.com/v6", "108d1196ef2513db69e7a424"));
        let convertion_rate = currency_converter.get_conversion_rate("EUR", "USD");
        assert_eq!(convertion_rate, 0.0);
    }
}