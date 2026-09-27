use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "inaturalist-cli")]
#[command(about = "A CLI tool for interacting with the iNaturalist API")]
#[command(version = "2.0")]
pub struct Cli {
    /// API requests per second (rate limiting)
    #[arg(short, long, default_value = "1.0")]
    pub rate_limit: f64,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Output format
    #[arg(long, value_enum, default_value = "table")]
    pub format: OutputFormat,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(ValueEnum, Clone)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get observation histogram statistics
    Histogram {
        /// Location ID (place_id)
        #[arg(short, long)]
        location: String,

        /// Interval (year, month, week, day, hour, month_of_year)
        #[arg(short, long, default_value = "month_of_year")]
        interval: String,

        /// Additional query parameters (key=value format)
        #[arg(short = 'p', long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,
    },

    /// Get observer statistics
    Observers {
        /// Location ID (place_id)
        #[arg(short, long)]
        location: String,

        /// Number of results
        #[arg(long, default_value = "20")]
        per_page: u32,

        /// Additional query parameters
        #[arg(short = 'p', long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,
    },

    /// Get identifier statistics
    Identifiers {
        /// Location ID (place_id)
        #[arg(short, long)]
        location: String,

        /// Number of results
        #[arg(long, default_value = "20")]
        per_page: u32,

        /// Additional query parameters
        #[arg(short = 'p', long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,
    },

    /// Get species counts
    SpeciesCounts {
        /// Location ID (place_id) - optional for global stats
        #[arg(short, long)]
        location: Option<String>,

        /// Taxon ID
        #[arg(short, long)]
        taxon_id: Option<String>,

        /// Number of results
        #[arg(long, default_value = "20")]
        per_page: u32,

        /// Additional query parameters
        #[arg(short = 'p', long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,
    },

    /// Search for taxa
    TaxaSearch {
        /// Search query
        #[arg(short, long)]
        query: String,

        /// Rank filter
        #[arg(short, long)]
        rank: Option<String>,

        /// Number of results
        #[arg(long, default_value = "20")]
        per_page: u32,
    },

    /// Autocomplete taxa
    TaxaAutocomplete {
        /// Search query
        #[arg(short, long)]
        query: String,

        /// Number of results
        #[arg(long, default_value = "10")]
        per_page: u32,
    },

    /// Search for places
    PlacesSearch {
        /// Search query
        #[arg(short, long)]
        query: String,
    },

    /// Autocomplete places
    PlacesAutocomplete {
        /// Search query
        #[arg(short, long)]
        query: String,
    },

    /// Find nearby places
    PlacesNearby {
        /// Northeast latitude
        #[arg(long)]
        nelat: f64,

        /// Northeast longitude
        #[arg(long)]
        nelng: f64,

        /// Southwest latitude
        #[arg(long)]
        swlat: f64,

        /// Southwest longitude
        #[arg(long)]
        swlng: f64,

        /// Name filter
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Search for projects
    ProjectsSearch {
        /// Search query
        #[arg(short, long)]
        query: String,

        /// Number of results
        #[arg(long, default_value = "20")]
        per_page: u32,
    },

    /// Process multiple locations in parallel
    Batch {
        /// CSV file containing location IDs (place_id) - one per line or column
        #[arg(short, long)]
        locations_file: String,

        /// Column name in CSV file containing place_ids (default: first column)
        #[arg(long, default_value = "place_id")]
        location_column: String,

        /// Type of statistics to gather
        #[arg(short, long, value_enum)]
        stat_type: StatType,

        /// Maximum number of parallel workers
        #[arg(short = 'w', long, default_value = "4")]
        max_workers: usize,

        /// Additional query parameters
        #[arg(short = 'p', long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,
    },

    /// Generic GET request to any iNaturalist API endpoint
    Get {
        /// API endpoint path (e.g., "observations", "taxa/123", "places/nearby")
        #[arg(short, long)]
        path: String,

        /// Additional query parameters (key=value format)
        #[arg(long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,
    },

    /// Batch generic GET requests to any iNaturalist API endpoint
    BatchGet {
        /// CSV file containing endpoint paths - one per line or column
        #[arg(short, long)]
        paths_file: String,

        /// Column name in CSV file containing paths (default: first column)
        #[arg(long, default_value = "path")]
        path_column: String,

        /// Maximum number of parallel workers
        #[arg(short = 'w', long, default_value = "4")]
        max_workers: usize,

        /// Additional query parameters (key=value format) - applied to all paths
        #[arg(long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,
    },

    /// Batch generic GET requests with individual parameters for each path
    BatchGetWithParams {
        /// CSV file containing endpoint paths with parameters
        /// Can have multiple columns: path, param1, param2, etc.
        #[arg(long)]
        paths_file: String,

        /// Column name in CSV file containing paths (default: "path")
        #[arg(long, default_value = "path")]
        path_column: String,

        /// Maximum number of parallel workers
        #[arg(short = 'w', long, default_value = "4")]
        max_workers: usize,
    },
}

#[derive(ValueEnum, Clone)]
pub enum StatType {
    Histogram,
    Observers,
    Identifiers,
    Species,
}

/// Parse a single key-value pair for parameters
fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}