use log::info;
use crate::client::INaturalistClient;
use crate::error::ClientError;
use super::args::{Commands, StatType, OutputFormat};

pub async fn execute_command(
    client: &INaturalistClient,
    command: Commands,
    format: OutputFormat,
) -> Result<(), ClientError> {
    match command {
        Commands::Histogram { location, interval, params } => {
            info!("Fetching histogram for location: {}", location);

            let mut builder = client.observations_histogram()
                .place_id(&location)
                .interval(&interval);

            // Add extra params
            for (key, value) in params {
                builder.params.insert(key, value);
            }

            let result = builder.fetch().await?;
            print_output(&result, format);
        }

        Commands::Observers { location, per_page, params: _ } => {
            info!("Fetching observers for location: {}", location);
            let result = client.observations_observers()
                .place_id(&location)
                .per_page(per_page)
                .fetch()
                .await?;
            print_output(&result, format);
        }

        Commands::Identifiers { location, per_page, params: _ } => {
            info!("Fetching identifiers for location: {}", location);
            let result = client.observations_identifiers()
                .place_id(&location)
                .per_page(per_page)
                .fetch()
                .await?;
            print_output(&result, format);
        }

        Commands::SpeciesCounts { location, taxon_id, per_page, params: _ } => {
            info!("Fetching species counts");
            let mut builder = client.observations_species_counts()
                .per_page(per_page);

            if let Some(loc) = location {
                builder = builder.place_id(&loc);
            }
            if let Some(taxon) = taxon_id {
                builder = builder.taxon_id(&taxon);
            }

            let result = builder.fetch().await?;
            print_output(&result, format);
        }

        Commands::TaxaSearch { query, rank, per_page } => {
            info!("Searching taxa: {}", query);
            let mut builder = client.taxa()
                .q(&query)
                .per_page(per_page);

            if let Some(r) = rank {
                builder = builder.rank(&r);
            }

            let result = builder.fetch().await?;
            print_output(&result, format);
        }

        Commands::TaxaAutocomplete { query, per_page } => {
            info!("Autocompleting taxa: {}", query);
            let result = client.taxa_autocomplete()
                .q(&query)
                .per_page(per_page)
                .fetch()
                .await?;
            print_output(&result, format);
        }

        Commands::PlacesSearch { query } => {
            info!("Searching places: {}", query);
            let result = client.places()
                .q(&query)
                .fetch()
                .await?;
            print_output(&result, format);
        }

        Commands::PlacesAutocomplete { query } => {
            info!("Autocompleting places: {}", query);
            let result = client.places_autocomplete()
                .q(&query)
                .fetch()
                .await?;
            print_output(&result, format);
        }

        Commands::PlacesNearby { nelat, nelng, swlat, swlng, name } => {
            info!("Finding nearby places");
            let mut builder = client.places_nearby()
                .nelat(nelat)
                .nelng(nelng)
                .swlat(swlat)
                .swlng(swlng);

            if let Some(n) = name {
                builder = builder.name(&n);
            }

            let result = builder.fetch().await?;
            print_output(&result, format);
        }

        Commands::ProjectsSearch { query, per_page } => {
            info!("Searching projects: {}", query);
            let result = client.projects()
                .q(&query)
                .per_page(per_page)
                .fetch()
                .await?;
            print_output(&result, format);
        }

        Commands::Batch { locations, stat_type, max_workers, params: _ } => {
            info!("Processing {} locations in parallel with {} workers", locations.len(), max_workers);

            match stat_type {
                StatType::Histogram => {
                    let results = client.batch_observations_histogram(locations, max_workers).await;
                    println!("Processed {} locations successfully", results.len());
                }
                StatType::Observers => {
                    let results = client.batch_observations_observers(locations, max_workers).await;
                    println!("Processed {} locations successfully", results.len());
                }
                StatType::Identifiers => {
                    let results = client.batch_observations_identifiers(locations, max_workers).await;
                    println!("Processed {} locations successfully", results.len());
                }
                StatType::Species => {
                    let results = client.batch_observations_species_counts(locations, max_workers).await;
                    println!("Processed {} locations successfully", results.len());
                }
            }
        }

        Commands::Get { path, params } => {
            info!("Fetching from endpoint: {}", path);

            let mut builder = client.get(&path);

            // Add extra params
            for (key, value) in params {
                builder.params.insert(key, value);
            }

            let result = builder.fetch().await?;
            print_output(&result, format);
        }

        Commands::BatchGet { paths, max_workers, params } => {
            info!("Processing {} paths in parallel with {} workers", paths.len(), max_workers);

            // Convert params to HashMap
            let params_map: std::collections::HashMap<String, String> = params.into_iter().collect();

            // Create path-params pairs
            let path_params: Vec<(String, std::collections::HashMap<String, String>)> = 
                paths.into_iter().map(|path| (path, params_map.clone())).collect();

            let results = client.batch_get_with_params(path_params, max_workers).await;
            println!("Processed {} paths successfully", results.len());
            
            // Print all results
            for (i, result) in results.iter().enumerate() {
                println!("\n--- Result {} ---", i + 1);
                print_output(result, format.clone());
            }
        }

        Commands::BatchGetWithParams { path_params, max_workers } => {
            info!("Processing {} path-param combinations in parallel with {} workers", path_params.len(), max_workers);

            // Parse path_params into (path, params) tuples
            let parsed: Vec<(String, std::collections::HashMap<String, String>)> = path_params
                .into_iter()
                .map(|pp| parse_path_with_params(&pp))
                .collect();

            let results = client.batch_get_with_params(parsed, max_workers).await;
            println!("Processed {} path-param combinations successfully", results.len());
            
            // Print all results
            for (i, result) in results.iter().enumerate() {
                println!("\n--- Result {} ---", i + 1);
                print_output(result, format.clone());
            }
        }
    }

    Ok(())
}

/// Parse a path with query parameters (e.g., "path?key1=value1&key2=value2")
fn parse_path_with_params(input: &str) -> (String, std::collections::HashMap<String, String>) {
    use std::collections::HashMap;
    
    let (path, query) = if let Some(pos) = input.find('?') {
        (&input[..pos], &input[pos+1..])
    } else {
        (input, "")
    };
    
    let mut params = HashMap::new();
    
    if !query.is_empty() {
        for pair in query.split('&') {
            if let Some(eq_pos) = pair.find('=') {
                let key = &pair[..eq_pos];
                let value = &pair[eq_pos+1..];
                params.insert(key.to_string(), value.to_string());
            } else if !pair.is_empty() {
                params.insert(pair.to_string(), String::new());
            }
        }
    }
    
    (path.to_string(), params)
}

fn print_output<T: serde::Serialize>(data: &T, format: OutputFormat) {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(data).unwrap());
        }
        OutputFormat::Table => {
            // Simple table output
            println!("{}", serde_json::to_string_pretty(data).unwrap());
        }
        OutputFormat::Csv => {
            // CSV output
            println!("{}", serde_json::to_string(data).unwrap());
        }
    }
}