use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::photo::{Photo, Sound};
use crate::models::taxon::Taxon;
use crate::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub id: u64,
    pub uuid: String,
    pub species_guess: Option<String>,
    pub description: Option<String>,
    pub observed_on: Option<NaiveDate>,
    pub observed_on_string: Option<String>,
    pub time_observed_at: Option<DateTime<Utc>>,
    pub quality_grade: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub positional_accuracy: Option<u32>,
    pub public_positional_accuracy: Option<u32>,
    pub geoprivacy: Option<String>,
    pub taxon_geoprivacy: Option<String>,
    pub coordinates_obscured: bool,
    pub place_guess: Option<String>,
    pub place_ids: Option<Vec<u32>>,
    pub taxon: Option<Taxon>,
    pub user: User,
    pub photos: Option<Vec<Photo>>,
    pub sounds: Option<Vec<Sound>>,
    pub captive: bool,
    pub num_identification_agreements: u32,
    pub num_identification_disagreements: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub uri: String,
    pub iconic_taxon_name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub faves_count: u32,
    pub num_comments: u32,
}

#[derive(Debug, Clone, Default)]
pub struct ObservationSearchParams {
    pub q: Option<String>,
    pub taxon_id: Option<Vec<u32>>,
    pub place_id: Option<Vec<u32>>,
    pub project_id: Option<Vec<u32>>,
    pub user_id: Option<Vec<u32>>,
    pub login: Option<Vec<String>>,
    pub quality_grade: Option<Vec<String>>,
    pub iconic_taxa: Option<Vec<String>>,
    pub rank: Option<Vec<String>>,
    pub order: Option<String>,
    pub order_by: Option<String>,
    pub photos: Option<bool>,
    pub sounds: Option<bool>,
    pub geo: Option<bool>,
    pub identified: Option<bool>,
    pub captive: Option<bool>,
    pub threatened: Option<bool>,
    pub endemic: Option<bool>,
    pub introduced: Option<bool>,
    pub native: Option<bool>,
    pub verifiable: Option<bool>,
    pub mappable: Option<bool>,
    pub popular: Option<bool>,
    pub id_above: Option<u64>,
    pub id_below: Option<u64>,
    pub observed_on: Option<String>,
    pub observed_on_year: Option<u16>,
    pub observed_on_month: Option<u8>,
    pub observed_on_day: Option<u8>,
    pub created_d1: Option<String>,
    pub created_d2: Option<String>,
    pub updated_since: Option<String>,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub radius: Option<f64>,
    pub nelat: Option<f64>,
    pub nelng: Option<f64>,
    pub swlat: Option<f64>,
    pub swlng: Option<f64>,
    pub acc_below: Option<u32>,
    pub page: Option<usize>,
    pub per_page: Option<usize>,
    pub extra_params: HashMap<String, String>,
}

impl ObservationSearchParams {
    pub fn to_hashmap(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        if let Some(ref q) = self.q { map.insert("q".to_string(), q.clone()); }
        if let Some(ref ids) = self.taxon_id { map.insert("taxon_id".to_string(), ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",")); }
        if let Some(ref ids) = self.place_id { map.insert("place_id".to_string(), ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",")); }
        if let Some(ref ids) = self.project_id { map.insert("project_id".to_string(), ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",")); }
        if let Some(ref ids) = self.user_id { map.insert("user_id".to_string(), ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",")); }
        if let Some(ref logins) = self.login { map.insert("login".to_string(), logins.join(",")); }
        if let Some(ref grades) = self.quality_grade { map.insert("quality_grade".to_string(), grades.join(",")); }
        if let Some(ref taxa) = self.iconic_taxa { map.insert("iconic_taxa".to_string(), taxa.join(",")); }
        if let Some(ref ranks) = self.rank { map.insert("rank".to_string(), ranks.join(",")); }
        if let Some(ref order) = self.order { map.insert("order".to_string(), order.clone()); }
        if let Some(ref order_by) = self.order_by { map.insert("order_by".to_string(), order_by.clone()); }
        if let Some(photos) = self.photos { map.insert("photos".to_string(), photos.to_string()); }
        if let Some(sounds) = self.sounds { map.insert("sounds".to_string(), sounds.to_string()); }
        if let Some(geo) = self.geo { map.insert("geo".to_string(), geo.to_string()); }
        if let Some(identified) = self.identified { map.insert("identified".to_string(), identified.to_string()); }
        if let Some(captive) = self.captive { map.insert("captive".to_string(), captive.to_string()); }
        if let Some(threatened) = self.threatened { map.insert("threatened".to_string(), threatened.to_string()); }
        if let Some(endemic) = self.endemic { map.insert("endemic".to_string(), endemic.to_string()); }
        if let Some(introduced) = self.introduced { map.insert("introduced".to_string(), introduced.to_string()); }
        if let Some(native) = self.native { map.insert("native".to_string(), native.to_string()); }
        if let Some(verifiable) = self.verifiable { map.insert("verifiable".to_string(), verifiable.to_string()); }
        if let Some(mappable) = self.mappable { map.insert("mappable".to_string(), mappable.to_string()); }
        if let Some(popular) = self.popular { map.insert("popular".to_string(), popular.to_string()); }
        if let Some(id) = self.id_above { map.insert("id_above".to_string(), id.to_string()); }
        if let Some(id) = self.id_below { map.insert("id_below".to_string(), id.to_string()); }
        if let Some(ref date) = self.observed_on { map.insert("observed_on".to_string(), date.clone()); }
        if let Some(year) = self.observed_on_year { map.insert("observed_on_year".to_string(), year.to_string()); }
        if let Some(month) = self.observed_on_month { map.insert("observed_on_month".to_string(), month.to_string()); }
        if let Some(day) = self.observed_on_day { map.insert("observed_on_day".to_string(), day.to_string()); }
        if let Some(ref d1) = self.created_d1 { map.insert("created_d1".to_string(), d1.clone()); }
        if let Some(ref d2) = self.created_d2 { map.insert("created_d2".to_string(), d2.clone()); }
        if let Some(ref since) = self.updated_since { map.insert("updated_since".to_string(), since.clone()); }
        if let Some(lat) = self.lat { map.insert("lat".to_string(), lat.to_string()); }
        if let Some(lng) = self.lng { map.insert("lng".to_string(), lng.to_string()); }
        if let Some(radius) = self.radius { map.insert("radius".to_string(), radius.to_string()); }
        if let Some(v) = self.nelat { map.insert("nelat".to_string(), v.to_string()); }
        if let Some(v) = self.nelng { map.insert("nelng".to_string(), v.to_string()); }
        if let Some(v) = self.swlat { map.insert("swlat".to_string(), v.to_string()); }
        if let Some(v) = self.swlng { map.insert("swlng".to_string(), v.to_string()); }
        if let Some(acc) = self.acc_below { map.insert("acc_below".to_string(), acc.to_string()); }
        if let Some(page) = self.page { map.insert("page".to_string(), page.to_string()); }
        if let Some(per_page) = self.per_page { map.insert("per_page".to_string(), per_page.to_string()); }

        for (k, v) in &self.extra_params {
            map.insert(k.clone(), v.clone());
        }

        map
    }
}
