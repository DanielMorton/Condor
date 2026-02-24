use std::collections::HashMap;
use reqwest::Client;
use serde::de::DeserializeOwned;
use tokio::time::sleep;
use log::{info, error};

use crate::client::rate_limiter::RateLimiter;
use crate::api::endpoints::Endpoints;
use crate::error::{ClientError, Result};
use crate::utils::backoff::exponential_backoff;

#[derive(Debug, Clone)]
pub struct RequestHandler {
    client: Client,
}

impl RequestHandler {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Generic GET request with automatic deserialization
    pub async fn get_with_params<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &HashMap<String, String>,
        rate_limiter: &RateLimiter,
    ) -> Result<T> {
        let url = Endpoints::full_url(endpoint);
        self.get_with_retry(&url, params, rate_limiter, 5).await
    }

    async fn get_data<T: DeserializeOwned>(
        &self,
        url: &str,
        params: &HashMap<String, String>,
    ) -> Result<T> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());

        let response = self.client
            .get(url)
            .query(params)
            .headers(headers)
            .send()
            .await?;

        if response.status().is_success() {
            let data: T = response.json().await?;
            Ok(data)
        } else if response.status() == 429 {
            Err(ClientError::RateLimit)
        } else {
            Err(ClientError::InvalidResponse(format!("HTTP error: {}", response.status())))
        }
    }

    async fn get_with_retry<T: DeserializeOwned>(
        &self,
        url: &str,
        params: &HashMap<String, String>,
        rate_limiter: &RateLimiter,
        max_retries: u32,
    ) -> Result<T> {
        let mut retry_count = 0;

        loop {
            rate_limiter.wait().await;

            match self.get_data(url, params).await {
                Ok(data) => return Ok(data),
                Err(ClientError::RateLimit) => {
                    retry_count += 1;
                    if retry_count > max_retries {
                        error!("Max retries exceeded for URL: {}", url);
                        return Err(ClientError::RateLimit);
                    }

                    let delay = exponential_backoff(retry_count, 1.0, 60.0);
                    info!("Rate limit hit for URL {}. Retrying in {:.2?}...", url, delay);
                    sleep(delay).await;
                }
                Err(e) => {
                    error!("Error for URL {}: {}", url, e);
                    return Err(e);
                }
            }
        }
    }

    /// Legacy compatibility method
    pub async fn get_stats(
        &self,
        endpoint_tag: &str,
        location: Option<&str>,
        extra_params: HashMap<String, String>,
        rate_limiter: &RateLimiter,
    ) -> Result<serde_json::Value> {
        let endpoint = Endpoints::full_url(endpoint_tag);
        let mut params = HashMap::new();
        params.insert("verifiable".to_string(), "true".to_string());

        if let Some(loc) = location {
            params.insert("place_id".to_string(), loc.to_string());
        }

        for (key, value) in extra_params {
            params.insert(key, value);
        }

        self.get_with_retry(&endpoint, &params, rate_limiter, 5).await
    }
}