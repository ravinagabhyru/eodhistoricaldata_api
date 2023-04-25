/*!
# eodhistoricaldata API

This project provides a set of functions to receive data from the
the eodhistoricaldata website via their [API](https://eodhistoricaldata.com/knowledgebase/). This project
is licensed under Apache 2.0 or MIT license (see files LICENSE-Apache2.0 and LICENSE-MIT).

# Usage
Please note that you need to have a registered account with eodhistoricaldata to
receive an individual API token. The most basic account is free but limited to
EOD Historical Data and LIVE/Realtime Data only and allows only 20 requests per day.

*/

use chrono::NaiveDate;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;

#[derive(Deserialize, Debug)]
pub struct RealTimeQuote {
    /// Ticker name
    pub code: String,
    /// UNIX timestamp convention, seconds passed sind 1st January 1970
    pub timestamp: u64,
    pub gmtoffset: i32,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: usize,
    #[serde(rename = "previousClose")]
    pub previous_close: f64,
    pub change: f64,
    pub change_p: f64,
}

#[derive(Deserialize, Debug)]
pub struct HistoricQuote {
    /// Quote date as string using the format `%Y-%m-%d`
    pub date: String,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub adjusted_close: f64,
    pub volume: Option<usize>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Dividend {
    pub currency: String,
    /// Quote date as string using the format `%Y-%m-%d`
    pub date: String,
    pub declaration_date: Option<String>,
    pub payment_date: String,
    pub period: String,
    pub record_date: String,
    pub unadjusted_value: f64,
    pub value: f64,
}

#[derive(Deserialize, Debug)]
pub struct Split {
    pub date: String,
    pub split: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AssetInformation {
    code: String,
    #[serde(rename = "Type")]
    asset_type: String,
    name: String,
    exchange: String,
    currency_code: String,
    currency_name: String,
    currency_symbol: String,
    country_name: String,
    #[serde(rename = "CountryISO")]
    country_iso: String,
    #[serde(rename = "ISIN")]
    isin: String,
    #[serde(rename = "LEI")]
    lei: String,
    primary_ticker: String,
    #[serde(rename = "CUSIP")]
    cusip: String,
    #[serde(rename = "CIK")]
    cik: String,
    #[serde(rename = "IPODate")]
    ipo_date: String,
    sector: String,
    industry: String,
    gic_sector: String,
    gic_group: String,
    gic_industry: String,
    gic_sub_industry: String,
    home_category: String,
    is_delisted: bool,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FundamentalsResponse {
    general: AssetInformation,
}


#[derive(Error, Debug)]
pub enum EodHistDataError {
    #[error("fetching the data from eodhistoricaldata failed with status code {0}")]
    FetchFailed(StatusCode),
    #[error("deserializing response from eodhistoricaldata failed")]
    DeserializeFailed(#[from] reqwest::Error),
    #[error("connection to eodhistoricaldata server failed")]
    ConnectionFailed(#[from] serde_json::Error),
}

pub struct EodHistConnector {
    url: &'static str,
    api_token: String,
}

impl EodHistConnector {
    /// Constructor for a new instance of EodHistConnector.
    /// token is the API token you got from eodhistoricaldata
    pub fn new(token: String) -> EodHistConnector {
        EodHistConnector {
            url: "https://eodhistoricaldata.com/api",
            api_token: token,
        }
    }

    /// Retrieve the latest quote for the given ticker
    pub async fn get_latest_quote(&self, ticker: &str) -> Result<RealTimeQuote, EodHistDataError> {
        let url: String = format!(
            "{}/real-time/{}?api_token={}&fmt=json",
            self.url, ticker, self.api_token
        );
        let resp = self.send_request(&url).await?;
        let quote: RealTimeQuote = serde_json::from_value(resp)?;
        Ok(quote)
    }

    /// Retrieve the quote history for the given ticker form date start to end (inklusive), if available
    pub async fn get_quote_history(
        &self,
        ticker: &str,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<HistoricQuote>, EodHistDataError> {
        let url: String = format!(
            "{}/eod/{}?from={}&to={}&api_token={}&period=d&fmt=json",
            self.url,
            ticker,
            start.format("%Y-%m-%d"),
            end.format("%Y-%m-%d"),
            self.api_token
        );
        let resp = self.send_request(&url).await?;
        let quotes: Vec<HistoricQuote> = serde_json::from_value(resp)?;
        Ok(quotes)
    }

    /// Retrieve the dividend history for the given ticker form date start to end (inclusive), if available
    pub async fn get_dividend_history(
        &self,
        ticker: &str,
        start: NaiveDate,
    ) -> Result<Vec<Dividend>, EodHistDataError> {
        let url: String = format!(
            "{}/div/{}?from={}&api_token={}&fmt=json",
            self.url,
            ticker,
            start.format("%Y-%m-%d"),
            self.api_token
        );
        let resp = self.send_request(&url).await?;
        let dividends: Vec<Dividend> = serde_json::from_value(resp)?;
        Ok(dividends)
    }

    /// Retrieve the split history for the given ticker form date start to end (inclusive), if available
    pub async fn get_split_history(
        &self,
        ticker: &str,
        start: NaiveDate,
    ) -> Result<Vec<Split>, EodHistDataError> {
        let url: String = format!(
            "{}/splits/{}?from={}&api_token={}&fmt=json",
            self.url,
            ticker,
            start.format("%Y-%m-%d"),
            self.api_token
        );
        let resp = self.send_request(&url).await?;
        let splits: Vec<Split> = serde_json::from_value(resp)?;
        Ok(splits)
    }

    /// Retrieve the fundamentals for the given ticker
    pub async fn get_asset_information(
        &self,
        ticker: &str,
    ) -> Result<AssetInformation, EodHistDataError> {
        let url: String = format!(
            "{}/fundamentals/{}?api_token={}",
            self.url,
            ticker,
            self.api_token
        );
        let resp = self.send_request(&url).await?;
        let fundamentals: FundamentalsResponse = serde_json::from_value(resp)?;
        Ok(fundamentals.general)
    }

    /// Send request to eodhistoricaldata server and transform response to JSON value
    async fn send_request(&self, url: &str) -> Result<Value, EodHistDataError> {
        let resp = reqwest::get(url).await?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await?),
            status => Err(EodHistDataError::FetchFailed(status)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[test]
    fn test_get_single_quote() {
        // Use the official test token
        let token = "OeAFFmMliFG5orCUuwAKQ8l4WWFQ67YX".to_string();
        let provider = EodHistConnector::new(token);
        let quote = tokio_test::block_on(provider.get_latest_quote("AAPL.US")).unwrap();

        assert_eq!(&quote.code, "AAPL.US");
    }

    #[test]
    fn test_get_quote_history() {
        // Use the official test token
        let token = "OeAFFmMliFG5orCUuwAKQ8l4WWFQ67YX".to_string();
        let provider = EodHistConnector::new(token);
        let start = NaiveDate::from_ymd_opt(2020, 01, 01).unwrap();
        let end = NaiveDate::from_ymd_opt(2020, 01, 31).unwrap();
        let quotes =
            tokio_test::block_on(provider.get_quote_history("AAPL.US", start, end)).unwrap();

        assert_eq!(quotes.len(), 21);
        assert_eq!(quotes[0].date, "2020-01-02");
        assert_eq!(quotes[quotes.len() - 1].date, "2020-01-31");
    }

    #[test]
    fn test_get_dividend_history() {
        // Use the official test token
        let token = "OeAFFmMliFG5orCUuwAKQ8l4WWFQ67YX".to_string();
        let provider = EodHistConnector::new(token);
        let start = NaiveDate::from_ymd_opt(2020, 01, 01).unwrap();
        let dividends =
            tokio_test::block_on(provider.get_dividend_history("AAPL.US", start)).unwrap();

        assert!(dividends.len() >= 4);
    }

    #[test]
    fn test_get_split_history() {
        // Use the official test token
        let token = "OeAFFmMliFG5orCUuwAKQ8l4WWFQ67YX".to_string();
        let provider = EodHistConnector::new(token);
        let start = NaiveDate::from_ymd_opt(2020, 01, 01).unwrap();
        let splits =
            tokio_test::block_on(provider.get_split_history("AAPL.US", start)).unwrap();

        assert!(splits.len() >= 1);
    }

    #[test]
    fn test_get_asset_information() {
        // Use the official test token
        let token = "OeAFFmMliFG5orCUuwAKQ8l4WWFQ67YX".to_string();
        let provider = EodHistConnector::new(token);
        let info = tokio_test::block_on(provider.get_asset_information("AAPL.US")).unwrap();

        assert_eq!(info.code, "AAPL");
        assert_eq!(info.asset_type, "Common Stock");
        assert!(info.name.contains("Apple Inc"));
        assert_eq!(info.exchange, "NASDAQ");
        assert_eq!(info.currency_code, "USD");
        assert_eq!(info.currency_name, "US Dollar");
        assert_eq!(info.currency_symbol, "$");
        assert_eq!(info.country_name, "USA");
        assert_eq!(info.country_iso, "US");
        assert_eq!(info.isin, "US0378331005");
        assert_eq!(info.lei, "HWUPKR0MPOU8FGXBT394");
        assert_eq!(info.primary_ticker, "AAPL.US");
        assert_eq!(info.cusip, "037833100");
        assert_eq!(info.cik, "320193");
        assert_eq!(info.ipo_date, "1980-12-12");
        assert_eq!(info.sector, "Technology");
        assert_eq!(info.industry, "Consumer Electronics");
        assert_eq!(info.gic_sector, "Information Technology");
        assert_eq!(info.gic_group, "Technology Hardware & Equipment");
        assert_eq!(info.gic_industry, "Technology Hardware, Storage & Peripherals");
        assert_eq!(info.gic_sub_industry, "Technology Hardware, Storage & Peripherals");
        assert_eq!(info.home_category, "Domestic");
        assert_eq!(info.is_delisted, false);
    }
}
