//! # eodhistoricaldata API
//!
//! This project provides a set of functions to receive data from the
//! the eodhistoricaldata website via their [API](https://eodhistoricaldata.com/knowledgebase/). This project
//! is licensed under Apache 2.0 or MIT license (see files LICENSE-Apache2.0 and LICENSE-MIT).
//!
//! # Usage
//! Please note that you need to have a registered account with eodhistoricaldata to
//! receive an individual API token. The most basic account is free but limited to
//! EOD Historical Data and LIVE/Realtime Data only and allows only 20 requests per day.
//!

use chrono::NaiveDate;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;
use std::fmt;
use tokio_compat_02::FutureExt;

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
    /// Quote date as string using the format `%Y-%m-%d`
    pub date: String,
    pub declaration_date: String,
    pub record_date: String,
    pub payment_date: String,
    pub period: String,
    pub value: f64,
    pub unadjusted_value: f64,
    pub currency: String,
}

#[derive(Debug)]
pub enum EodHistDataError {
    FetchFailed(StatusCode),
    DeserializeFailed,
    ConnectionFailed,
}

impl std::error::Error for EodHistDataError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        Some(self)
    }
}

impl fmt::Display for EodHistDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FetchFailed(status) => write!(f, "fetchin the data from eodhistoricaldata failed: with status code {}", status),
            Self::DeserializeFailed => write!(f, "deserializing response from eodhistoricaldata failed"),
            Self::ConnectionFailed => write!(f, "connection to eodhistoricaldata server failed"),
        }
    }
}

/// Container for connection paramters to edohistoricaldata server
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
        println!("{:?}", url);
        let resp = self.send_request(&url).await?;
        let quote: RealTimeQuote =
            serde_json::from_value(resp).map_err(|_| EodHistDataError::DeserializeFailed)?;
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
        println!("{:?}", url);
        let resp = self.send_request(&url).await?;
        let quotes: Vec<HistoricQuote> =
            serde_json::from_value(resp).map_err(|_| EodHistDataError::DeserializeFailed)?;
        Ok(quotes)
    }

    /// Retrieve the quote history for the given ticker form date start to end (inklusive), if available
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
        println!("{:?}", url);
        let resp = self.send_request(&url).await?;
        let dividends: Vec<Dividend> =
            serde_json::from_value(resp).map_err(|_| EodHistDataError::DeserializeFailed)?;
        Ok(dividends)
    }
    
    /// Send request to eodhistoricaldata server and transform response to JSON value
    async fn send_request(&self, url: &str) -> Result<Value, EodHistDataError> {
        let resp = reqwest::get(url).compat().await;
        if resp.is_err() {
            return Err(EodHistDataError::ConnectionFailed);
        }
        let resp = resp.unwrap();
        match resp.status() {
            StatusCode::OK => match resp.json().await {
                Ok(json) => Ok(json),
                _ => Err(EodHistDataError::DeserializeFailed),
            },

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
        let start = NaiveDate::from_ymd(2020, 01, 01);
        let end = NaiveDate::from_ymd(2020, 01, 31);
        let quotes = tokio_test::block_on(provider.get_quote_history("AAPL.US", start, end)).unwrap();

        assert_eq!(quotes.len(), 21);
        assert_eq!(quotes[0].date, "2020-01-02");
        assert_eq!(quotes[quotes.len() - 1].date, "2020-01-31");
    }

    #[test]
    fn test_get_dividen_history() {
        // Use the official test token
        let token = "OeAFFmMliFG5orCUuwAKQ8l4WWFQ67YX".to_string();
        let provider = EodHistConnector::new(token);
        let start = NaiveDate::from_ymd(2020, 01, 01);
        let dividends = tokio_test::block_on(provider.get_dividend_history("AAPL.US", start)).unwrap();

        assert!(dividends.len() >= 4);
    }
}
