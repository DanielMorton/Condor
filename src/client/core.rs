use std::sync::Arc;
use reqwest::Client;
use std::time::Duration;
use futures::future::join_all;
use log::error;

use crate::client::rate_limiter::RateLimiter;
use crate::api::requests::RequestHandler;
use crate::error::{ClientError, Result};

// Import all the builder structs from endpoints
use crate::endpoints::definitions::*;
use crate::endpoints::GenericGetBuilder;

#[derive(Debug, Clone)]
pub struct INaturalistClient {
    pub(crate) request_handler: RequestHandler,
    pub(crate) rate_limiter: RateLimiter,
}

impl INaturalistClient {
    pub fn new(requests_per_second: f64) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(ClientError::Http)?;

        Ok(Self {
            request_handler: RequestHandler::new(client),
            rate_limiter: RateLimiter::new(requests_per_second),
        })
    }

    // Observations endpoints
    pub fn observations_histogram(&self) -> ObservationsHistogramBuilder {
        ObservationsHistogramBuilder::new(self.clone())
    }

    pub fn observations_observers(&self) -> ObservationsObserversBuilder {
        ObservationsObserversBuilder::new(self.clone())
    }

    pub fn observations_identifiers(&self) -> ObservationsIdentifiersBuilder {
        ObservationsIdentifiersBuilder::new(self.clone())
    }

    pub fn observations_species_counts(&self) -> ObservationsSpeciesCountsBuilder {
        ObservationsSpeciesCountsBuilder::new(self.clone())
    }

    // Identifications endpoints
    pub fn identifications(&self) -> IdentificationsBuilder {
        IdentificationsBuilder::new(self.clone())
    }

    pub fn identifications_species_counts(&self) -> IdentificationsSpeciesCountsBuilder {
        IdentificationsSpeciesCountsBuilder::new(self.clone())
    }

    // Taxa endpoints
    pub fn taxa(&self) -> TaxaBuilder {
        TaxaBuilder::new(self.clone())
    }

    pub fn taxa_autocomplete(&self) -> TaxaAutocompleteBuilder {
        TaxaAutocompleteBuilder::new(self.clone())
    }

    // Places endpoints
    pub fn places(&self) -> PlacesBuilder {
        PlacesBuilder::new(self.clone())
    }

    pub fn places_autocomplete(&self) -> PlacesAutocompleteBuilder {
        PlacesAutocompleteBuilder::new(self.clone())
    }

    pub fn places_nearby(&self) -> PlacesNearbyBuilder {
        PlacesNearbyBuilder::new(self.clone())
    }

    // Projects endpoints
    pub fn projects(&self) -> ProjectsBuilder {
        ProjectsBuilder::new(self.clone())
    }

    pub fn projects_autocomplete(&self) -> ProjectsAutocompleteBuilder {
        ProjectsAutocompleteBuilder::new(self.clone())
    }

    // Users endpoints
    pub fn users_autocomplete(&self) -> UsersAutocompleteBuilder {
        UsersAutocompleteBuilder::new(self.clone())
    }

    // Search endpoint
    pub fn search(&self) -> SearchBuilder {
        SearchBuilder::new(self.clone())
    }

    // Generic GET endpoint for any iNaturalist API path
    pub fn get(&self, path: impl Into<String>) -> GenericGetBuilder {
        GenericGetBuilder::new(self.clone()).path(path)
    }

    /// Generic GET with path and parameters
    pub fn get_with_params(&self, path: impl Into<String>, params: std::collections::HashMap<String, String>) -> GenericGetBuilder {
        GenericGetBuilder::new(self.clone()).path(path).with_params(params)
    }

    // Batch processing methods
    pub async fn batch_observations_species_counts(
        &self,
        locations: Vec<String>,
        max_workers: usize,
    ) -> Vec<crate::models::SpeciesCountsResponse> {
        self.process_locations_parallel(
            locations,
            {
                let client = self.clone();
                move |location| {
                    let client = client.clone();
                    async move {
                        client.observations_species_counts()
                            .place_id(&location)
                            .fetch()
                            .await
                    }
                }
            },
            Some(max_workers),
        ).await
    }

    pub async fn batch_observations_observers(
        &self,
        locations: Vec<String>,
        max_workers: usize,
    ) -> Vec<crate::models::ObserversResponse> {
        self.process_locations_parallel(
            locations,
            {
                let client = self.clone();
                move |location| {
                    let client = client.clone();
                    async move {
                        client.observations_observers()
                            .place_id(&location)
                            .fetch()
                            .await
                    }
                }
            },
            Some(max_workers),
        ).await
    }

    pub async fn batch_observations_histogram(
        &self,
        locations: Vec<String>,
        max_workers: usize,
    ) -> Vec<crate::models::HistogramResponse> {
        self.process_locations_parallel(
            locations,
            {
                let client = self.clone();
                move |location| {
                    let client = client.clone();
                    async move {
                        client.observations_histogram()
                            .place_id(&location)
                            .fetch()
                            .await
                    }
                }
            },
            Some(max_workers),
        ).await
    }

    pub async fn batch_observations_identifiers(
        &self,
        locations: Vec<String>,
        max_workers: usize,
    ) -> Vec<crate::models::IdentifiersResponse> {
        self.process_locations_parallel(
            locations,
            {
                let client = self.clone();
                move |location| {
                    let client = client.clone();
                    async move {
                        client.observations_identifiers()
                            .place_id(&location)
                            .fetch()
                            .await
                    }
                }
            },
            Some(max_workers),
        ).await
    }

    // Generic batch processing for any GET endpoint
    pub async fn batch_get(
        &self,
        paths: Vec<String>,
        max_workers: usize,
    ) -> Vec<serde_json::Value> {
        self.process_locations_parallel(
            paths,
            {
                let client = self.clone();
                move |path| {
                    let client = client.clone();
                    async move {
                        client.get(&path).fetch().await
                    }
                }
            },
            Some(max_workers),
        ).await
    }

    // Generic batch processing with parameters for any GET endpoint
    pub async fn batch_get_with_params(
        &self,
        path_params: Vec<(String, std::collections::HashMap<String, String>)>,
        max_workers: usize,
    ) -> Vec<serde_json::Value> {
        use std::sync::Arc;
        use futures::future::join_all;

        let max_workers = std::cmp::min(max_workers, path_params.len());
        let semaphore = Arc::new(tokio::sync::Semaphore::new(max_workers));

        let tasks: Vec<_> = path_params.into_iter().map(|(path, params)| {
            let semaphore = semaphore.clone();
            let client = self.clone();

            tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                client.get(path).with_params(params).fetch().await
            })
        }).collect();

        let results = join_all(tasks).await;

        results.into_iter()
            .filter_map(|result| match result {
                Ok(Ok(data)) => Some(data),
                Ok(Err(e)) => {
                    error!("Task failed: {}", e);
                    None
                }
                Err(e) => {
                    error!("Task panicked: {}", e);
                    None
                }
            })
            .collect()
    }

    // Keep the existing parallel processing method
    pub async fn process_locations_parallel<T, F, Fut>(
        &self,
        locations: Vec<String>,
        process_fn: F,
        max_workers: Option<usize>,
    ) -> Vec<T>
    where
        F: Fn(String) -> Fut + Send + Sync + Clone + 'static,
        Fut: std::future::Future<Output = Result<T>> + Send,
        T: Send + 'static,
    {
        let max_workers = max_workers.unwrap_or_else(|| std::cmp::min(4, locations.len()));
        let semaphore = Arc::new(tokio::sync::Semaphore::new(max_workers));

        let tasks: Vec<_> = locations.into_iter().map(|location| {
            let semaphore = semaphore.clone();
            let process_fn = process_fn.clone();

            tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                process_fn(location).await
            })
        }).collect();

        let results = join_all(tasks).await;

        results.into_iter()
            .filter_map(|result| match result {
                Ok(Ok(data)) => Some(data),
                Ok(Err(e)) => {
                    error!("Task failed: {}", e);
                    None
                }
                Err(e) => {
                    error!("Task panicked: {}", e);
                    None
                }
            })
            .collect()
    }
}