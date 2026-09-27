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

/// Recursively flatten JSON into key-value pairs with dot notation
/// Arrays are kept as JSON strings to prevent column explosion
fn flatten_json_value(
    prefix: &str,
    value: &serde_json::Value,
    result: &mut std::collections::HashMap<String, String>,
    depth: usize,
) {
    const MAX_DEPTH: usize = 5;
    
    if depth > MAX_DEPTH {
        // At max depth, just store the JSON as a string
        result.insert(prefix.to_string(), value.to_string());
        return;
    }
    
    match value {
        serde_json::Value::Object(map) => {
            if map.is_empty() {
                result.insert(prefix.to_string(), "{}".to_string());
            } else {
                for (key, val) in map {
                    let new_prefix = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", prefix, key)
                    };
                    flatten_json_value(&new_prefix, val, result, depth + 1);
                }
            }
        }
        serde_json::Value::Array(arr) => {
            // For small arrays of primitives, expand them as comma-separated values
            // For complex arrays, store as JSON string
            if arr.len() <= 5 && arr.iter().all(|v| matches!(v, serde_json::Value::String(_) | serde_json::Value::Number(_) | serde_json::Value::Bool(_))) {
                let values: Vec<String> = arr.iter().map(|v| match v {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    _ => String::new(),
                }).collect();
                result.insert(prefix.to_string(), values.join(","));
            } else {
                // Store complex arrays as JSON strings
                result.insert(prefix.to_string(), serde_json::to_string(arr).unwrap());
            }
        }
        serde_json::Value::String(s) => {
            result.insert(prefix.to_string(), s.clone());
        }
        serde_json::Value::Number(n) => {
            result.insert(prefix.to_string(), n.to_string());
        }
        serde_json::Value::Bool(b) => {
            result.insert(prefix.to_string(), b.to_string());
        }
        serde_json::Value::Null => {
            result.insert(prefix.to_string(), "".to_string());
        }
    }
}

/// Extract headers from a collection of flat maps
fn get_headers(rows: &[std::collections::HashMap<String, String>]) -> Vec<String> {
    use std::collections::HashSet;
    
    let mut headers = HashSet::new();
    for row in rows {
        for key in row.keys() {
            headers.insert(key.clone());
        }
    }
    let mut sorted: Vec<String> = headers.into_iter().collect();
    sorted.sort();
    sorted
}

/// Convert rows to CSV format
fn rows_to_csv(rows: &[std::collections::HashMap<String, String>], headers: &[String]) -> String {
    use csv::WriterBuilder;
    
    let mut wtr = WriterBuilder::new().from_writer(vec![]);
    
    // Write header
    wtr.write_record(headers).unwrap();
    
    // Write each row
    for row in rows {
        let mut record: Vec<String> = Vec::new();
        for header in headers {
            record.push(row.get(header).cloned().unwrap_or_default());
        }
        wtr.write_record(&record).unwrap();
    }
    
    String::from_utf8(wtr.into_inner().unwrap()).unwrap()
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
            // Convert to JSON value first
            let json_value = serde_json::to_value(data).unwrap();
            
            // Extract the results array from ApiResponse wrapper
            let results: Vec<&serde_json::Value> = match &json_value {
                serde_json::Value::Object(map) if map.contains_key("results") => {
                    if let Some(serde_json::Value::Array(arr)) = map.get("results") {
                        arr.iter().collect()
                    } else {
                        vec![&json_value]
                    }
                }
                serde_json::Value::Array(arr) => {
                    arr.iter().collect()
                }
                _ => {
                    vec![&json_value]
                }
            };
            
            // Convert each result item to a row, flattening nested objects
            let mut rows: Vec<std::collections::HashMap<String, String>> = Vec::new();
            for result in results {
                let mut flat_row = std::collections::HashMap::new();
                flatten_json_value("", result, &mut flat_row, 0);
                rows.push(flat_row);
            }
            let headers = get_headers(&rows);
            let csv = rows_to_csv(&rows, &headers);
            println!("{}", csv);
        }
    }
}