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
    pub volume: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct EODQuote {
    /// Quote date as string using the format `%Y-%m-%d`
    pub code: String,
    pub exchange_short_name: String,
    pub date: String,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub adjusted_close: f64,
    pub volume: Option<f64>,
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
    pub code: String,
    #[serde(rename = "Type")]
    pub asset_type: Option<String>,
    pub name: Option<String>,
    pub exchange: Option<String>,
    pub currency_code: Option<String>,
    pub currency_name: Option<String>,
    pub currency_symbol: Option<String>,
    pub country_name: Option<String>,
    #[serde(rename = "CountryISO")]
    pub country_iso: Option<String>,
    #[serde(rename = "ISIN")]
    pub isin: Option<String>,
    #[serde(rename = "LEI")]
    pub lei: Option<String>,
    pub primary_ticker: Option<String>,
    #[serde(rename = "CUSIP")]
    pub cusip: Option<String>,
    #[serde(rename = "CIK")]
    pub cik: Option<String>,
    #[serde(rename = "IPODate")]
    pub ipo_date: Option<String>,
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub gic_sector: Option<String>,
    pub gic_group: Option<String>,
    pub gic_industry: Option<String>,
    pub gic_sub_industry: Option<String>,
    pub home_category: Option<String>,
    pub is_delisted: Option<bool>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Highlights {
    pub market_capitalization: Option<f64>,
    pub market_capitalization_mln: Option<f64>,
    #[serde(rename = "EBITDA")]
    pub ebitda: Option<f64>,
    #[serde(rename = "PERatio")]
    pub pe_ratio: Option<f64>,
    #[serde(rename = "PEGRatio")]
    pub peg_ratio: Option<f64>,
    pub wall_street_target_price: Option<f64>,
    pub book_value: Option<f64>,
    pub dividend_share: Option<f64>,
    pub dividend_yield: Option<f64>,
    pub earnings_share: Option<f64>,
    #[serde(rename = "EPSEstimateCurrentYear")]
    pub eps_estimate_current_year: Option<f64>,
    #[serde(rename = "EPSEstimateNextYear")]
    pub eps_estimate_next_year: Option<f64>,
    #[serde(rename = "EPSEstimateNextQuarter")]
    pub eps_estimate_next_quarter: Option<f64>,
    #[serde(rename = "EPSEstimateCurrentQuarter")]
    pub eps_estimate_current_quarter: Option<f64>,
    pub most_recent_quarter: Option<String>,
    pub profit_margin: Option<f64>,
    #[serde(rename = "OperatingMarginTTM")]
    pub operating_margin_ttm: Option<f64>,
    #[serde(rename = "ReturnOnAssetsTTM")]
    pub return_on_assets_ttm: Option<f64>,
    #[serde(rename = "ReturnOnEquityTTM")]
    pub return_on_equity_ttm: Option<f64>,
    #[serde(rename = "RevenueTTM")]
    pub revenue_ttm: Option<f64>,
    #[serde(rename = "RevenuePerShareTTM")]
    pub revenue_per_share_ttm: Option<f64>,
    #[serde(rename = "QuarterlyRevenueGrowthYOY")]
    pub quarterly_revenue_growth_yoy: Option<f64>,
    #[serde(rename = "GrossProfitTTM")]
    pub gross_profit_ttm: Option<f64>,
    #[serde(rename = "DilutedEpsTTM")]
    pub diluted_eps_ttm: Option<f64>,
    #[serde(rename = "QuarterlyEarningsGrowthYOY")]
    pub quarterly_earnings_growth_yoy: Option<f64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Valuation {
    #[serde(rename = "TrailingPE")]
    pub trailing_pe: Option<f64>,
    #[serde(rename = "ForwardPE")]
    pub forward_pe: Option<f64>,
    #[serde(rename = "PriceSalesTTM")]
    pub price_sales_ttm: Option<f64>,
    #[serde(rename = "PriceBookMRQ")]
    pub price_book_mrq: Option<f64>,
    pub enterprise_value: Option<f64>,
    pub enterprise_value_revenue: Option<f64>,
    pub enterprise_value_ebitda: Option<f64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SharesStats {
    pub shares_outstanding: Option<u64>,
    pub shares_float: Option<u64>,
    pub percent_insiders: Option<f64>,
    pub percent_institutions: Option<f64>,
    pub shares_short: Option<u64>,
    pub shares_short_prior_month: Option<u64>,
    pub short_ratio: Option<f64>,
    pub short_percent_outstanding: Option<f64>,
    pub short_percent_float: Option<f64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FundamentalsResponse {
    pub general: AssetInformation,
    pub highlights: Highlights,
    pub valuation: Valuation,
    pub shares_stats: SharesStats,
}


#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Exchange {
    pub name: String,
    pub code: String,
    pub operating_mic: String,
    pub country: String,
    pub currency: String,
    #[serde(rename = "CountryISO2")]
    pub country_iso2: String,
    #[serde(rename = "CountryISO3")]
    pub country_iso3: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ticker {
    pub code: String,
    pub name: String,
    pub country: String,
    pub exchange: String,
    pub currency: String,
    #[serde(rename = "Type")]
    pub asset_type: String,

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
    pub async fn get_fundamentals_information(
        &self,
        ticker: &str,
    ) -> Result<FundamentalsResponse, EodHistDataError> {
        let url: String = format!(
            "{}/fundamentals/{}?api_token={}",
            self.url,
            ticker,
            self.api_token
        );
        let resp = self.send_request(&url).await?;
        let fundamentals: FundamentalsResponse = serde_json::from_value(resp)?;
        Ok(fundamentals)
    }

    /// Retrieve the list of supported exchanges
    pub async fn get_exchanges(&self) -> Result<Vec<Exchange>, EodHistDataError> {
        let url: String = format!("{}/exchanges-list/?api_token={}", self.url, self.api_token);
        let resp = self.send_request(&url).await?;
        let exchanges: Vec<Exchange> = serde_json::from_value(resp)?;
        Ok(exchanges)
    }

    /// Retrieve the list of tickers for the given exchange
    pub async fn get_exchange_tickers(&self, exchange: &str) -> Result<Vec<Ticker>, EodHistDataError> {
        let url: String = format!(
            "{}/exchange-symbol-list/{}?api_token={}",
            self.url, exchange, self.api_token
        );
        let resp = self.send_request(&url).await?;
        let tickers: Vec<Ticker> = serde_json::from_value(resp)?;
        Ok(tickers)
    }

    /// Retrieve fundamentals data in bulk
    pub async fn get_fundamentals_bulk(
        &self,
        exchange: &str,
        offset: u32,
        limit: u32,
    ) -> Result<Vec<FundamentalsResponse>, EodHistDataError> {
        let url: String = format!(
            "{}/bulk-fundamentals/{}?api_token={}&offset={}&limit={}&fmt=json",
            self.url,
            exchange,
            self.api_token,
            offset,
            limit
        );
        let resp = self.send_request(&url).await?;
        let fundamentals: Vec<FundamentalsResponse> = serde_json::from_value(resp)?;
        Ok(fundamentals)
    }

    /// Retrieve eod bulk data for the given exchange and date
    pub async fn get_eod_bulk(
        &self,
        exchange: &str,
        date: NaiveDate,
    ) -> Result<Vec<EODQuote>, EodHistDataError> {
        let url: String = format!(
            "{}/eod-bulk-last-day/{}?api_token={}&date={}&fmt=json",
            self.url,
            exchange,
            self.api_token,
            date.format("%Y-%m-%d")
        );
        let resp = self.send_request(&url).await?;
        let quotes: Vec<EODQuote> = serde_json::from_value(resp)?;
        Ok(quotes)
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
        let fundamentals = tokio_test::block_on(provider.get_fundamentals_information("AAPL.US")).unwrap();
        let info = fundamentals.general;

        assert_eq!(info.code, "AAPL");
        assert_eq!(info.asset_type.unwrap(), "Common Stock");
        assert!(info.name.unwrap().contains("Apple Inc"));
        assert_eq!(info.exchange.unwrap(), "NASDAQ");
        assert_eq!(info.currency_code.unwrap(), "USD");
        assert_eq!(info.currency_name.unwrap(), "US Dollar");
        assert_eq!(info.currency_symbol.unwrap(), "$");
        assert_eq!(info.country_name.unwrap(), "USA");
        assert_eq!(info.country_iso.unwrap(), "US");
        assert_eq!(info.isin.unwrap(), "US0378331005");
        assert_eq!(info.lei.unwrap(), "HWUPKR0MPOU8FGXBT394");
        assert_eq!(info.primary_ticker.unwrap(), "AAPL.US");
        assert_eq!(info.cusip.unwrap(), "037833100");
        assert_eq!(info.cik.unwrap(), "320193");
        assert_eq!(info.ipo_date.unwrap(), "1980-12-12");
        assert_eq!(info.sector.unwrap(), "Technology");
        assert_eq!(info.industry.unwrap(), "Consumer Electronics");
        assert_eq!(info.gic_sector.unwrap(), "Information Technology");
        assert_eq!(info.gic_group.unwrap(), "Technology Hardware & Equipment");
        assert_eq!(info.gic_industry.unwrap(), "Technology Hardware, Storage & Peripherals");
        assert_eq!(info.gic_sub_industry.unwrap(), "Technology Hardware, Storage & Peripherals");
        assert_eq!(info.home_category.unwrap(), "Domestic");
        assert_eq!(info.is_delisted.unwrap(), false);
    }
}
