use crate::cli::identifier_stats::IdentifierStatsCommand;
use crate::cli::location_stats::LocationStatsCommand;
use crate::cli::observer_stats::ObserverStatsCommand;
use crate::cli::species_stats::SpeciesStatsCommand;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "condor")]
#[command(about = "A CLI tool for interacting with the iNaturalist API")]
#[command(version = "1.0")]
pub struct Cli {
    /// API requests per second (rate limiting)
    #[arg(short, long, default_value = "1.0")]
    pub rate_limit: f64,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Output format
    #[arg(short = 'f', long, default_value = "csv", value_parser = ["csv", "json", "pretty"])]
    pub format: String,

    /// Output file (default: stdout)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get statistics for locations
    #[command(subcommand)]
    Stats(StatsCommands),

    /// Observation commands
    #[command(subcommand, alias = "obs")]
    Observations(ObservationCommands),

    /// Identification commands
    #[command(subcommand, alias = "id")]
    Identifications(IdentificationCommands),

    /// Taxon commands
    #[command(subcommand, alias = "taxa")]
    Taxa(TaxonCommands),

    /// Place commands
    #[command(subcommand, alias = "places")]
    Places(PlaceCommands),

    /// Project commands
    #[command(subcommand, alias = "projects")]
    Projects(ProjectCommands),

    /// User commands
    #[command(subcommand, alias = "users")]
    Users(UserCommands),

    /// Unified search
    Search(SearchCommand),

    /// Controlled terms
    #[command(alias = "controlled-terms")]
    ControlledTerms(ControlledTermsCommand),
}

#[derive(Subcommand)]
pub enum StatsCommands {
    /// Get location histogram statistics (monthly observation counts)
    LocationStats(LocationStatsCommand),
    /// Get observer statistics for a location
    ObserverStats(ObserverStatsCommand),
    /// Get identifier statistics for a location
    IdentifierStats(IdentifierStatsCommand),
    /// Get species statistics for a location
    SpeciesStats(SpeciesStatsCommand),
}

#[derive(Subcommand)]
pub enum ObservationCommands {
    /// Search observations with filters
    Search(ObservationSearchArgs),
    /// Get a single observation by ID
    Get(ObservationGetArgs),
}

#[derive(Subcommand)]
pub enum IdentificationCommands {
    /// Search identifications with filters
    Search(IdentificationSearchArgs),
    /// Get a single identification by ID
    Get(IdentificationGetArgs),
}

#[derive(Subcommand)]
pub enum TaxonCommands {
    /// Search taxa
    Search(TaxonSearchArgs),
    /// Get a single taxon by ID
    Get(TaxonGetArgs),
    /// Autocomplete taxon names
    Autocomplete(TaxonAutocompleteArgs),
    /// Suggest taxa from image or location
    Suggest(TaxonSuggestArgs),
}

#[derive(Subcommand)]
pub enum PlaceCommands {
    /// Search places
    Search(PlaceSearchArgs),
    /// Get a single place by ID
    Get(PlaceGetArgs),
    /// Find places near a bounding box
    Nearby(PlaceNearbyArgs),
    /// Autocomplete place names
    Autocomplete(PlaceAutocompleteArgs),
}

#[derive(Subcommand)]
pub enum ProjectCommands {
    /// Search projects
    Search(ProjectSearchArgs),
    /// Get a single project by ID
    Get(ProjectGetArgs),
    /// Get project members
    Members(ProjectMembersArgs),
}

#[derive(Subcommand)]
pub enum UserCommands {
    /// Get a single user by ID
    Get(UserGetArgs),
    /// Get current user
    Me,
    /// Autocomplete user names
    Autocomplete(UserAutocompleteArgs),
}

// === Observation Args ===
#[derive(clap::Args)]
pub struct ObservationSearchArgs {
    #[arg(short, long)]
    pub taxon_id: Option<Vec<u32>>,
    #[arg(short, long)]
    pub place_id: Option<Vec<u32>>,
    #[arg(long)]
    pub user_id: Option<Vec<u32>>,
    #[arg(long)]
    pub quality_grade: Option<Vec<String>>,
    #[arg(long)]
    pub observed_on: Option<String>,
    #[arg(long)]
    pub photos: bool,
    #[arg(long)]
    pub sounds: bool,
    #[arg(long)]
    pub geo: bool,
    #[arg(long)]
    pub verifiable: bool,
    #[arg(long)]
    pub lat: Option<f64>,
    #[arg(long)]
    pub lng: Option<f64>,
    #[arg(long)]
    pub radius: Option<f64>,
    #[arg(short, long, default_value = "30")]
    pub per_page: usize,
    #[arg(long)]
    pub max_pages: Option<usize>,
}

#[derive(clap::Args)]
pub struct ObservationGetArgs {
    pub id: u64,
}

// === Identification Args ===
#[derive(clap::Args)]
pub struct IdentificationSearchArgs {
    #[arg(long)]
    pub observation_id: Option<Vec<u64>>,
    #[arg(long)]
    pub taxon_id: Option<Vec<u32>>,
    #[arg(long)]
    pub user_id: Option<Vec<u32>>,
    #[arg(long)]
    pub current: bool,
    #[arg(short, long, default_value = "30")]
    pub per_page: usize,
}

#[derive(clap::Args)]
pub struct IdentificationGetArgs {
    pub id: u64,
}

// === Taxon Args ===
#[derive(clap::Args)]
pub struct TaxonSearchArgs {
    #[arg(short, long)]
    pub q: Option<String>,
    #[arg(long)]
    pub taxon_id: Option<Vec<u32>>,
    #[arg(long)]
    pub rank: Option<Vec<String>>,
    #[arg(short, long, default_value = "30")]
    pub per_page: usize,
}

#[derive(clap::Args)]
pub struct TaxonGetArgs {
    pub id: u32,
}

#[derive(clap::Args)]
pub struct TaxonAutocompleteArgs {
    pub query: String,
    #[arg(short, long, default_value = "10")]
    pub per_page: usize,
}

#[derive(clap::Args)]
pub struct TaxonSuggestArgs {
    #[arg(long)]
    pub image_url: Option<String>,
    #[arg(long)]
    pub lat: Option<f64>,
    #[arg(long)]
    pub lng: Option<f64>,
}

// === Place Args ===
#[derive(clap::Args)]
pub struct PlaceSearchArgs {
    #[arg(short, long)]
    pub q: Option<String>,
    #[arg(long)]
    pub place_type: Option<Vec<String>>,
    #[arg(long)]
    pub ancestor_place_id: Option<u32>,
    #[arg(short, long, default_value = "30")]
    pub per_page: usize,
}

#[derive(clap::Args)]
pub struct PlaceGetArgs {
    pub id: u32,
}

#[derive(clap::Args)]
pub struct PlaceNearbyArgs {
    #[arg(long)]
    pub nelat: f64,
    #[arg(long)]
    pub nelng: f64,
    #[arg(long)]
    pub swlat: f64,
    #[arg(long)]
    pub swlng: f64,
    #[arg(long)]
    pub name: Option<String>,
}

#[derive(clap::Args)]
pub struct PlaceAutocompleteArgs {
    pub query: String,
    #[arg(short, long, default_value = "10")]
    pub per_page: usize,
}

// === Project Args ===
#[derive(clap::Args)]
pub struct ProjectSearchArgs {
    #[arg(short, long)]
    pub q: Option<String>,
    #[arg(long)]
    pub place_id: Option<Vec<u32>>,
    #[arg(long)]
    pub taxon_id: Option<Vec<u32>>,
    #[arg(long)]
    pub user_id: Option<Vec<u32>>,
    #[arg(short, long, default_value = "30")]
    pub per_page: usize,
}

#[derive(clap::Args)]
pub struct ProjectGetArgs {
    pub id: u32,
}

#[derive(clap::Args)]
pub struct ProjectMembersArgs {
    pub id: u32,
    #[arg(short, long, default_value = "30")]
    pub per_page: usize,
}

// === User Args ===
#[derive(clap::Args)]
pub struct UserGetArgs {
    pub id: u32,
}

#[derive(clap::Args)]
pub struct UserAutocompleteArgs {
    pub query: String,
    #[arg(short, long, default_value = "10")]
    pub per_page: usize,
}

// === Search Args ===
#[derive(clap::Args)]
pub struct SearchCommand {
    pub query: String,
    #[arg(long, value_delimiter = ',')]
    pub sources: Option<Vec<String>>,
    #[arg(short, long, default_value = "30")]
    pub per_page: usize,
}

// === Controlled Terms Args ===
#[derive(clap::Args)]
pub struct ControlledTermsCommand {
    #[arg(long)]
    pub taxon_id: Option<u32>,
}

/// Parse a single key-value pair for parameters
pub fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}

/// Helper function to read locations from CSV file
pub fn read_locations_from_csv(
    csv_file: &PathBuf,
    column_name: &str,
) -> crate::error::Result<Vec<u32>> {
    use csv::ReaderBuilder;
    use std::fs::File;

    let file = File::open(csv_file)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    let headers = reader.headers()?.clone();
    let column_index = headers
        .iter()
        .position(|h| h == column_name)
        .ok_or_else(|| format!("Column '{}' not found in CSV", column_name))?;

    let mut locations = Vec::new();
    for result in reader.records() {
        let record = result?;
        if let Some(value) = record.get(column_index) {
            match value.trim().parse::<u32>() {
                Ok(location_id) => locations.push(location_id),
                Err(e) => eprintln!("Warning: Could not parse '{}' as u32: {}", value, e),
            }
        }
    }

    if locations.is_empty() {
        return Err(format!("No valid location IDs found in column '{}'", column_name).into());
    }

    Ok(locations)
}

/// Helper function to get locations from either CLI args or CSV file
pub fn get_locations(
    locations: &[u32],
    csv_file: &Option<PathBuf>,
    csv_column: &str,
) -> crate::error::Result<Vec<u32>> {
    if let Some(csv_path) = csv_file {
        read_locations_from_csv(csv_path, csv_column)
    } else if !locations.is_empty() {
        Ok(locations.to_vec())
    } else {
        Err("Either --locations or --csv-file must be provided".into())
    }
}
