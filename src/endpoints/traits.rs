use async_trait::async_trait;
use crate::error::Result;
use std::collections::HashMap;

/// Trait for endpoints that support fetching
#[async_trait]
pub trait Fetchable {
    type Output;

    async fn fetch(self) -> Result<Self::Output>;
}

/// Trait for endpoints that support pagination
pub trait Paginatable {
    fn page(self, page: u32) -> Self;
    fn per_page(self, per_page: u32) -> Self;
}

/// Trait for endpoints that support location filtering
pub trait LocationFilterable {
    fn place_id(self, place_id: impl Into<String>) -> Self;
    fn lat_lng(self, lat: f64, lng: f64, radius: Option<f64>) -> Self;
    fn bounding_box(self, nelat: f64, nelng: f64, swlat: f64, swlng: f64) -> Self;
}

/// Trait for endpoints that support taxon filtering
pub trait TaxonFilterable {
    fn taxon_id(self, taxon_id: impl Into<String>) -> Self;
    fn rank(self, rank: impl Into<String>) -> Self;
    fn iconic_taxa(self, iconic_taxa: impl Into<String>) -> Self;
}

/// Trait for endpoints that support time filtering
pub trait TimeFilterable {
    fn observed_on(self, date: impl Into<String>) -> Self;
    fn observed_between(self, start: impl Into<String>, end: impl Into<String>) -> Self;
    fn created_on(self, date: impl Into<String>) -> Self;
    fn created_between(self, start: impl Into<String>, end: impl Into<String>) -> Self;
}

/// Trait for endpoints that support quality filtering
pub trait QualityFilterable {
    fn quality_grade(self, grade: impl Into<String>) -> Self;
    fn verifiable(self, verifiable: bool) -> Self;
    fn identified(self, identified: bool) -> Self;
}

/// Trait for endpoints that support media filtering
pub trait MediaFilterable {
    fn photos(self, has_photos: bool) -> Self;
    fn sounds(self, has_sounds: bool) -> Self;
}

/// Trait for endpoints that can be customized with raw parameters
pub trait Customizable {
    fn with_params(self, params: HashMap<String, String>) -> Self;
    fn add_param(self, key: impl Into<String>, value: impl Into<String>) -> Self;
}