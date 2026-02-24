pub struct Endpoints;

impl Endpoints {
    pub const BASE_URL: &'static str = "https://api.inaturalist.org/v1";

    pub fn observations_histogram() -> &'static str {
        "observations/histogram"
    }

    pub fn observations_observers() -> &'static str {
        "observations/observers"
    }

    pub fn observations_identifiers() -> &'static str {
        "observations/identifiers"
    }

    pub fn observations_species_counts() -> &'static str {
        "observations/species_counts"
    }

    pub fn identifications() -> &'static str {
        "identifications"
    }

    pub fn identifications_species_counts() -> &'static str {
        "identifications/species_counts"
    }

    pub fn identifications_identifiers() -> &'static str {
        "identifications/identifiers"
    }

    pub fn identifications_observers() -> &'static str {
        "identifications/observers"
    }

    pub fn taxa() -> &'static str {
        "taxa"
    }

    pub fn taxa_autocomplete() -> &'static str {
        "taxa/autocomplete"
    }

    pub fn places() -> &'static str {
        "places"
    }

    pub fn places_autocomplete() -> &'static str {
        "places/autocomplete"
    }

    pub fn places_nearby() -> &'static str {
        "places/nearby"
    }

    pub fn projects() -> &'static str {
        "projects"
    }

    pub fn projects_autocomplete() -> &'static str {
        "projects/autocomplete"
    }

    pub fn users_autocomplete() -> &'static str {
        "users/autocomplete"
    }

    pub fn search() -> &'static str {
        "search"
    }

    pub fn full_url(endpoint: &str) -> String {
        format!("{}/{}", Self::BASE_URL, endpoint)
    }
}