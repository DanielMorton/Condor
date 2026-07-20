use log::info;

use crate::api::requests::{ApiResponse, RequestHandler};
use crate::client::rate_limiter::RateLimiter;
use crate::error::Result;
use std::collections::HashMap;

/// Fetch all pages of a paginated endpoint
pub async fn fetch_all_pages<T, F, Fut>(
    mut fetch_page: F,
    per_page: usize,
) -> Result<Vec<T>>
where
    F: FnMut(usize) -> Fut,
    Fut: std::future::Future<Output = Result<ApiResponse<T>>>,
{
    let mut all_results = Vec::new();
    let mut page = 1;

    loop {
        let response = fetch_page(page).await?;
        let batch_size = response.results.len();
        let total = response.total_results.unwrap_or(0);

        all_results.extend(response.results);

        if all_results.len() >= total || batch_size < per_page || batch_size == 0 {
            info!("Fetched {} total results across {} pages", all_results.len(), page);
            break;
        }

        page += 1;
    }

    Ok(all_results)
}

/// Fetch a single page
pub async fn fetch_page<T: serde::de::DeserializeOwned>(
    handler: &RequestHandler,
    endpoint: &str,
    params: &mut HashMap<String, String>,
    page: usize,
    per_page: usize,
    rate_limiter: &RateLimiter,
) -> Result<ApiResponse<T>> {
    params.insert("page".to_string(), page.to_string());
    params.insert("per_page".to_string(), per_page.to_string());
    handler.get::<T>(endpoint, params, rate_limiter).await
}
