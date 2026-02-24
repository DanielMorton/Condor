use crate::define_param_group;

// Common observation search parameters
define_param_group! {
    ObservationParams {
        place_id: String,
        taxon_id: String,
        user_id: String,
        quality_grade: String,
        verifiable: bool,
        captive: bool,
        photos: bool,
        sounds: bool,
        identified: bool,
        geo: bool,
        d1: String,
        d2: String,
        created_d1: String,
        created_d2: String,
        per_page: u32,
        page: u32,
        order_by: String,
        order: String,
    }
}

// Pagination parameters
define_param_group! {
    PaginationParams {
        page: u32,
        per_page: u32,
    }
}

// Date range parameters
define_param_group! {
    DateRangeParams {
        d1: String,
        d2: String,
        created_d1: String,
        created_d2: String,
    }
}

// Location parameters
define_param_group! {
    LocationParams {
        place_id: String,
        lat: f64,
        lng: f64,
        radius: f64,
        nelat: f64,
        nelng: f64,
        swlat: f64,
        swlng: f64,
    }
}