use futures::future::join_all;
use log::error;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use crate::api::endpoints::Endpoints;
use crate::api::requests::RequestHandler;
use crate::client::rate_limiter::RateLimiter;
use crate::client::pagination::{fetch_all_pages, fetch_page};
use crate::error::{ClientError, Result};
use crate::models::{
    ControlledTerm, Identification, IdentificationSearchParams, Observation,
    ObservationHistogramStats, ObservationIdentifierStats, ObservationObserverStats,
    ObservationSearchParams, ObservationSpeciesStats, Place, PlaceSearchParams,
    Project, ProjectSearchParams, SearchParams, SearchResult, Taxon, TaxonSuggestion,
    User,
};

#[derive(Debug, Clone)]
pub struct INaturalistClient {
    request_handler: RequestHandler,
    rate_limiter: RateLimiter,
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

    // ============================================================
    // EXISTING METHODS (PRESERVED)
    // ============================================================

    pub async fn get_observation_histogram(
        &self,
        location: u32,
        extra_params: HashMap<String, String>,
    ) -> Result<Vec<ObservationHistogramStats>> {
        let results = self
            .request_handler
            .get_stats(
                Endpoints::OBSERVATIONS_HISTOGRAM,
                Some(location),
                extra_params,
                &self.rate_limiter,
            )
            .await?;

        ObservationHistogramStats::from_histogram_response(&results, location)
    }

    pub async fn get_observer_stats(
        &self,
        location: u32,
        extra_params: HashMap<String, String>,
    ) -> Result<Vec<ObservationObserverStats>> {
        let results = self
            .request_handler
            .get_stats(
                Endpoints::OBSERVATIONS_OBSERVERS,
                Some(location),
                extra_params,
                &self.rate_limiter,
            )
            .await?;

        ObservationObserverStats::from_response(&results, location)
    }

    pub async fn get_identifier_stats(
        &self,
        location: u32,
        extra_params: HashMap<String, String>,
    ) -> Result<Vec<ObservationIdentifierStats>> {
        let results = self
            .request_handler
            .get_stats(
                Endpoints::OBSERVATIONS_IDENTIFIERS,
                Some(location),
                extra_params,
                &self.rate_limiter,
            )
            .await?;

        ObservationIdentifierStats::from_response(&results, location)
    }

    pub async fn get_species_stats(
        &self,
        location: Option<u32>,
        extra_params: HashMap<String, String>,
    ) -> Result<Vec<ObservationSpeciesStats>> {
        let results = self
            .request_handler
            .get_stats(
                Endpoints::OBSERVATIONS_SPECIES_COUNTS,
                location,
                extra_params,
                &self.rate_limiter,
            )
            .await?;

        ObservationSpeciesStats::from_response(&results, location)
    }

    pub async fn process_locations_parallel<T, F, Fut>(
        &self,
        locations: Vec<u32>,
        process_fn: F,
        max_workers: Option<usize>,
    ) -> Vec<T>
    where
        F: Fn(u32) -> Fut + Send + Sync + Clone + 'static,
        Fut: std::future::Future<Output = Result<T>> + Send,
        T: Send + 'static,
    {
        let max_workers = max_workers.unwrap_or_else(|| std::cmp::min(4, locations.len()));
        let semaphore = Arc::new(tokio::sync::Semaphore::new(max_workers));

        let tasks: Vec<_> = locations
            .into_iter()
            .map(|location| {
                let semaphore = semaphore.clone();
                let process_fn = process_fn.clone();

                tokio::spawn(async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    process_fn(location).await
                })
            })
            .collect();

        let results = join_all(tasks).await;

        results
            .into_iter()
            .filter_map(|result| match result {
                Ok(Ok(data)) => Some(data),
                Ok(Err(e)) => {
                    error!("Task failed: {e}");
                    None
                }
                Err(e) => {
                    error!("Task panicked: {e}");
                    None
                }
            })
            .collect()
    }

    // ============================================================
    // NEW: OBSERVATIONS
    // ============================================================

    /// Search observations with filters. Auto-paginates through all results.
    pub async fn search_observations(
        &self,
        params: &ObservationSearchParams,
    ) -> Result<Vec<Observation>> {
        let endpoint = Endpoints::url(Endpoints::OBSERVATIONS);
        let mut query = params.to_hashmap();
        let per_page = params.per_page.unwrap_or(30);

        fetch_all_pages(
            |page| {
                let handler = self.request_handler.clone();
                let rate_limiter = self.rate_limiter.clone();
                let mut q = query.clone();
                let ep = endpoint.clone();
                async move {
                    fetch_page::<Observation>(&handler, &ep, &mut q, page, per_page, &rate_limiter).await
                }
            },
            per_page,
        )
            .await
    }

    /// Get a single observation by ID
    pub async fn get_observation(&self, id: u64) -> Result<Observation> {
        self.request_handler
            .get_by_id::<Observation>(Endpoints::OBSERVATIONS, id, &HashMap::new(), &self.rate_limiter)
            .await
    }

    /// Get observation updates
    pub async fn get_observation_updates(
        &self,
        params: Option<&HashMap<String, String>>,
    ) -> Result<Vec<Observation>> {
        let endpoint = Endpoints::url(Endpoints::OBSERVATIONS_UPDATES);
        let query = params.cloned().unwrap_or_default();
        let response = self
            .request_handler
            .get::<Observation>(&endpoint, &query, &self.rate_limiter)
            .await?;
        Ok(response.results)
    }

    /// Get deleted observations
    pub async fn get_deleted_observations(
        &self,
        params: Option<&HashMap<String, String>>,
    ) -> Result<Vec<Observation>> {
        let endpoint = Endpoints::url(Endpoints::OBSERVATIONS_DELETED);
        let query = params.cloned().unwrap_or_default();
        let response = self
            .request_handler
            .get::<Observation>(&endpoint, &query, &self.rate_limiter)
            .await?;
        Ok(response.results)
    }

    // ============================================================
    // NEW: IDENTIFICATIONS
    // ============================================================

    /// Search identifications with filters. Auto-paginates.
    pub async fn search_identifications(
        &self,
        params: &IdentificationSearchParams,
    ) -> Result<Vec<Identification>> {
        let endpoint = Endpoints::url(Endpoints::IDENTIFICATIONS);
        let mut query = params.to_hashmap();
        let per_page = params.per_page.unwrap_or(30);

        fetch_all_pages(
            |page| {
                let handler = self.request_handler.clone();
                let rate_limiter = self.rate_limiter.clone();
                let mut q = query.clone();
                let ep = endpoint.clone();
                async move {
                    fetch_page::<Identification>(&handler, &ep, &mut q, page, per_page, &rate_limiter).await
                }
            },
            per_page,
        )
            .await
    }

    /// Get a single identification by ID
    pub async fn get_identification(&self, id: u64) -> Result<Identification> {
        self.request_handler
            .get_by_id::<Identification>(Endpoints::IDENTIFICATIONS, id, &HashMap::new(), &self.rate_limiter)
            .await
    }

    // ============================================================
    // NEW: TAXA
    // ============================================================

    /// Search taxa. Auto-paginates.
    pub async fn search_taxa(
        &self,
        q: Option<&str>,
        taxon_id: Option<Vec<u32>>,
        rank: Option<Vec<String>>,
        per_page: Option<usize>,
    ) -> Result<Vec<Taxon>> {
        let endpoint = Endpoints::url(Endpoints::TAXA);
        let mut params = HashMap::new();
        if let Some(q) = q {
            params.insert("q".to_string(), q.to_string());
        }
        if let Some(ids) = taxon_id {
            params.insert(
                "taxon_id".to_string(),
                ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","),
            );
        }
        if let Some(ranks) = rank {
            params.insert("rank".to_string(), ranks.join(","));
        }

        let per_page = per_page.unwrap_or(30);

        fetch_all_pages(
            |page| {
                let handler = self.request_handler.clone();
                let rate_limiter = self.rate_limiter.clone();
                let mut p = params.clone();
                let ep = endpoint.clone();
                async move {
                    fetch_page::<Taxon>(&handler, &ep, &mut p, page, per_page, &rate_limiter).await
                }
            },
            per_page,
        )
            .await
    }

    /// Get a single taxon by ID
    pub async fn get_taxon(&self, id: u32) -> Result<Taxon> {
        self.request_handler
            .get_by_id::<Taxon>(Endpoints::TAXA, id, &HashMap::new(), &self.rate_limiter)
            .await
    }

    /// Autocomplete taxon names
    pub async fn taxa_autocomplete(
        &self,
        q: &str,
        per_page: Option<usize>,
    ) -> Result<Vec<Taxon>> {
        let endpoint = Endpoints::url(Endpoints::TAXA_AUTOCOMPLETE);
        let mut params = HashMap::new();
        params.insert("q".to_string(), q.to_string());
        if let Some(n) = per_page {
            params.insert("per_page".to_string(), n.to_string());
        }

        let response = self
            .request_handler
            .get::<Taxon>(&endpoint, &params, &self.rate_limiter)
            .await?;
        Ok(response.results)
    }

    /// Suggest taxa from image URL or coordinates
    pub async fn suggest_taxa(
        &self,
        image_url: Option<&str>,
        lat: Option<f64>,
        lng: Option<f64>,
    ) -> Result<Vec<TaxonSuggestion>> {
        let endpoint = Endpoints::url(Endpoints::TAXA_SUGGEST);
        let mut params = HashMap::new();
        if let Some(url) = image_url {
            params.insert("image_url".to_string(), url.to_string());
        }
        if let Some(lat) = lat {
            params.insert("lat".to_string(), lat.to_string());
        }
        if let Some(lng) = lng {
            params.insert("lng".to_string(), lng.to_string());
        }

        let response = self
            .request_handler
            .get::<TaxonSuggestion>(&endpoint, &params, &self.rate_limiter)
            .await?;
        Ok(response.results)
    }

    // ============================================================
    // NEW: PLACES
    // ============================================================

    /// Search places. Auto-paginates.
    pub async fn search_places(
        &self,
        params: &PlaceSearchParams,
    ) -> Result<Vec<Place>> {
        let endpoint = Endpoints::url(Endpoints::PLACES);
        let mut query = params.to_hashmap();
        let per_page = params.per_page.unwrap_or(30);

        fetch_all_pages(
            |page| {
                let handler = self.request_handler.clone();
                let rate_limiter = self.rate_limiter.clone();
                let mut q = query.clone();
                let ep = endpoint.clone();
                async move {
                    fetch_page::<Place>(&handler, &ep, &mut q, page, per_page, &rate_limiter).await
                }
            },
            per_page,
        )
            .await
    }

    /// Get a single place by ID
    pub async fn get_place(&self, id: u32) -> Result<Place> {
        self.request_handler
            .get_by_id::<Place>(Endpoints::PLACES, id, &HashMap::new(), &self.rate_limiter)
            .await
    }

    /// Find places near a bounding box
    pub async fn places_nearby(
        &self,
        nelat: f64,
        nelng: f64,
        swlat: f64,
        swlng: f64,
        name: Option<&str>,
    ) -> Result<Vec<Place>> {
        let endpoint = Endpoints::url(Endpoints::PLACES_NEARBY);
        let mut params = HashMap::new();
        params.insert("nelat".to_string(), nelat.to_string());
        params.insert("nelng".to_string(), nelng.to_string());
        params.insert("swlat".to_string(), swlat.to_string());
        params.insert("swlng".to_string(), swlng.to_string());
        if let Some(n) = name {
            params.insert("name".to_string(), n.to_string());
        }

        let response = self
            .request_handler
            .get::<Place>(&endpoint, &params, &self.rate_limiter)
            .await?;
        Ok(response.results)
    }

    /// Autocomplete place names
    pub async fn places_autocomplete(
        &self,
        q: &str,
        per_page: Option<usize>,
    ) -> Result<Vec<Place>> {
        let endpoint = Endpoints::url(Endpoints::PLACES_AUTOCOMPLETE);
        let mut params = HashMap::new();
        params.insert("q".to_string(), q.to_string());
        if let Some(n) = per_page {
            params.insert("per_page".to_string(), n.to_string());
        }

        let response = self
            .request_handler
            .get::<Place>(&endpoint, &params, &self.rate_limiter)
            .await?;
        Ok(response.results)
    }

    // ============================================================
    // NEW: PROJECTS
    // ============================================================

    /// Search projects. Auto-paginates.
    pub async fn search_projects(
        &self,
        params: &ProjectSearchParams,
    ) -> Result<Vec<Project>> {
        let endpoint = Endpoints::url(Endpoints::PROJECTS);
        let mut query = params.to_hashmap();
        let per_page = params.per_page.unwrap_or(30);

        fetch_all_pages(
            |page| {
                let handler = self.request_handler.clone();
                let rate_limiter = self.rate_limiter.clone();
                let mut q = query.clone();
                let ep = endpoint.clone();
                async move {
                    fetch_page::<Project>(&handler, &ep, &mut q, page, per_page, &rate_limiter).await
                }
            },
            per_page,
        )
            .await
    }

    /// Get a single project by ID
    pub async fn get_project(&self, id: u32) -> Result<Project> {
        self.request_handler
            .get_by_id::<Project>(Endpoints::PROJECTS, id, &HashMap::new(), &self.rate_limiter)
            .await
    }

    /// Get project members
    pub async fn get_project_members(
        &self,
        id: u32,
        page: Option<usize>,
        per_page: Option<usize>,
    ) -> Result<Vec<User>> {
        let endpoint = Endpoints::url(&Endpoints::PROJECT_MEMBERS.replace("{id}", &id.to_string()));
        let mut params = HashMap::new();
        if let Some(p) = page {
            params.insert("page".to_string(), p.to_string());
        }
        if let Some(pp) = per_page {
            params.insert("per_page".to_string(), pp.to_string());
        }

        let response = self
            .request_handler
            .get::<User>(&endpoint, &params, &self.rate_limiter)
            .await?;
        Ok(response.results)
    }

    // ============================================================
    // NEW: USERS
    // ============================================================

    /// Get a single user by ID
    pub async fn get_user(&self, id: u32) -> Result<User> {
        self.request_handler
            .get_by_id::<User>(Endpoints::USERS, id, &HashMap::new(), &self.rate_limiter)
            .await
    }

    /// Get current user (requires auth, but endpoint is read-only)
    pub async fn get_current_user(&self) -> Result<User> {
        let endpoint = Endpoints::url(Endpoints::USERS_ME);
        let params = HashMap::new();
        let response = self
            .request_handler
            .get::<User>(&endpoint, &params, &self.rate_limiter)
            .await?;
        response
            .results
            .into_iter()
            .next()
            .ok_or_else(|| ClientError::NotFound("Current user not found".to_string()))
    }

    /// Autocomplete user names
    pub async fn users_autocomplete(
        &self,
        q: &str,
        per_page: Option<usize>,
    ) -> Result<Vec<User>> {
        let endpoint = Endpoints::url(Endpoints::USERS_AUTOCOMPLETE);
        let mut params = HashMap::new();
        params.insert("q".to_string(), q.to_string());
        if let Some(n) = per_page {
            params.insert("per_page".to_string(), n.to_string());
        }

        let response = self
            .request_handler
            .get::<User>(&endpoint, &params, &self.rate_limiter)
            .await?;
        Ok(response.results)
    }

    // ============================================================
    // NEW: SEARCH
    // ============================================================

    /// Unified search across taxa, places, projects, users
    pub async fn search(&self, params: &SearchParams) -> Result<Vec<SearchResult>> {
        let endpoint = Endpoints::url(Endpoints::SEARCH);
        let mut query = params.to_hashmap();
        let per_page = params.per_page.unwrap_or(30);

        fetch_all_pages(
            |page| {
                let handler = self.request_handler.clone();
                let rate_limiter = self.rate_limiter.clone();
                let mut q = query.clone();
                let ep = endpoint.clone();
                async move {
                    fetch_page::<SearchResult>(&handler, &ep, &mut q, page, per_page, &rate_limiter).await
                }
            },
            per_page,
        )
            .await
    }

    // ============================================================
    // NEW: CONTROLLED TERMS
    // ============================================================

    /// Get controlled terms (optionally filtered by taxon)
    pub async fn get_controlled_terms(
        &self,
        taxon_id: Option<u32>,
    ) -> Result<Vec<ControlledTerm>> {
        let endpoint = Endpoints::url(Endpoints::CONTROLLED_TERMS);
        let mut params = HashMap::new();
        if let Some(id) = taxon_id {
            params.insert("taxon_id".to_string(), id.to_string());
        }

        let response = self
            .request_handler
            .get::<ControlledTerm>(&endpoint, &params, &self.rate_limiter)
            .await?;
        Ok(response.results)
    }
}
