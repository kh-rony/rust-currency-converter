# Rust Currency Converter CLI using ExchangeRate API

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Configure ExchangeRate API key](#configure-exchangeRate-api-key)
- [Release and Run](#release-and-run)
- [Examples](#examples)

## Installation

1. Ensure you have Rust installed. If not, you can install it by following the official [Rust installation guide](https://www.rust-lang.org/tools/install).
2. Run the following command to fetch and build the dependencies:
   ```bash
   cargo update
   ```

## Usage

```
rust-currency-converter <from_currency> <to_currency> <amount>
```

## Configure ExchangeRate API key

1. Go to [www.exchangerate-api.com](https://www.exchangerate-api.com/)
2. Create a free account using your email. You will get your own API key.
3. Create an `.env` file.
4. Provide your API key in `.env` file as `EXCHANGERATE_API_KEY`, (see `.env.example`)

## Release and Run

```
bash release.sh
./rust-currency-converter eur usd 1000
```

## Examples
```
./rust-currency-converter eur usd 1000
Converted amount: 1092.300 USD
[ Exchange Rate: 1 EUR = 1.0923 USD ]
```
