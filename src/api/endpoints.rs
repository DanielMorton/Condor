pub struct Endpoints;

impl Endpoints {
    pub const BASE_URL: &'static str = "https://api.inaturalist.org/v1";

    // === Observations ===
    pub const OBSERVATIONS: &'static str = "observations";
    pub const OBSERVATIONS_HISTOGRAM: &'static str = "observations/histogram";
    pub const OBSERVATIONS_OBSERVERS: &'static str = "observations/observers";
    pub const OBSERVATIONS_IDENTIFIERS: &'static str = "observations/identifiers";
    pub const OBSERVATIONS_SPECIES_COUNTS: &'static str = "observations/species_counts";
    pub const OBSERVATIONS_UPDATES: &'static str = "observations/updates";
    pub const OBSERVATIONS_DELETED: &'static str = "observations/deleted";

    // === Identifications ===
    pub const IDENTIFICATIONS: &'static str = "identifications";

    // === Taxa ===
    pub const TAXA: &'static str = "taxa";
    pub const TAXA_AUTOCOMPLETE: &'static str = "taxa/autocomplete";
    pub const TAXA_SUGGEST: &'static str = "taxa/suggest";

    // === Places ===
    pub const PLACES: &'static str = "places";
    pub const PLACES_NEARBY: &'static str = "places/nearby";
    pub const PLACES_AUTOCOMPLETE: &'static str = "places/autocomplete";

    // === Projects ===
    pub const PROJECTS: &'static str = "projects";
    pub const PROJECT_MEMBERS: &'static str = "projects/{id}/members";

    // === Users ===
    pub const USERS: &'static str = "users";
    pub const USERS_AUTOCOMPLETE: &'static str = "users/autocomplete";
    pub const USERS_ME: &'static str = "users/me";

    // === Search ===
    pub const SEARCH: &'static str = "search";

    // === Controlled Terms ===
    pub const CONTROLLED_TERMS: &'static str = "controlled_terms";

    pub fn url(endpoint: &str) -> String {
        format!("{}/{}", Self::BASE_URL, endpoint)
    }

    pub fn with_id(endpoint: &str, id: impl std::fmt::Display) -> String {
        format!("{}/{}", endpoint, id)
    }
}
