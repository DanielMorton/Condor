use crate::endpoint_builder;
use crate::models::*;

// Observations endpoints
endpoint_builder! {
    name: observations_histogram,
    path: "observations/histogram",
    response: HistogramResponse,
    params: {
        place_id: String,
        taxon_id: String,
        interval: String,
        date_field: String,
        d1: String,
        d2: String,
    }
}

endpoint_builder! {
    name: observations_observers,
    path: "observations/observers",
    response: ObserversResponse,
    params: {
        place_id: String,
        taxon_id: String,
        d1: String,
        d2: String,
        per_page: u32,
    }
}

endpoint_builder! {
    name: observations_identifiers,
    path: "observations/identifiers",
    response: IdentifiersResponse,
    params: {
        place_id: String,
        taxon_id: String,
        d1: String,
        d2: String,
        per_page: u32,
    }
}

endpoint_builder! {
    name: observations_species_counts,
    path: "observations/species_counts",
    response: SpeciesCountsResponse,
    params: {
        place_id: String,
        taxon_id: String,
        quality_grade: String,
        per_page: u32,
    }
}

// Identifications endpoints
endpoint_builder! {
    name: identifications,
    path: "identifications",
    response: IdentificationsResponse,
    params: {
        user_id: String,
        place_id: String,
        taxon_id: String,
        current: bool,
        per_page: u32,
    }
}

endpoint_builder! {
    name: identifications_species_counts,
    path: "identifications/species_counts",
    response: SpeciesCountsResponse,
    params: {
        user_id: String,
        place_id: String,
        taxon_id: String,
        per_page: u32,
    }
}

// Taxa endpoints
endpoint_builder! {
    name: taxa,
    path: "taxa",
    response: TaxaResponse,
    params: {
        q: String,
        rank: String,
        is_active: bool,
        per_page: u32,
    }
}

endpoint_builder! {
    name: taxa_autocomplete,
    path: "taxa/autocomplete",
    response: TaxaResponse,
    params: {
        q: String,
        rank: String,
        per_page: u32,
    }
}

// Places endpoints
endpoint_builder! {
    name: places,
    path: "places",
    response: PlacesResponse,
    params: {
        q: String,
        admin_level: i32,
    }
}

endpoint_builder! {
    name: places_autocomplete,
    path: "places/autocomplete",
    response: PlacesResponse,
    params: {
        q: String,
    }
}

endpoint_builder! {
    name: places_nearby,
    path: "places/nearby",
    response: PlacesResponse,
    params: {
        nelat: f64,
        nelng: f64,
        swlat: f64,
        swlng: f64,
        name: String,
    }
}

// Projects endpoints
endpoint_builder! {
    name: projects,
    path: "projects",
    response: ProjectsResponse,
    params: {
        q: String,
        lat: f64,
        lng: f64,
        radius: f64,
        featured: bool,
        noteworthy: bool,
        per_page: u32,
    }
}

endpoint_builder! {
    name: projects_autocomplete,
    path: "projects/autocomplete",
    response: ProjectsResponse,
    params: {
        q: String,
    }
}

// Users endpoints
endpoint_builder! {
    name: users_autocomplete,
    path: "users/autocomplete",
    response: UsersResponse,
    params: {
        q: String,
        project_id: String,
        per_page: u32,
    }
}

// Search endpoint
endpoint_builder! {
    name: search,
    path: "search",
    response: SearchResponse,
    params: {
        q: String,
        sources: String,
        place_id: String,
        per_page: u32,
    }
}

