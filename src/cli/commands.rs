use crate::cli::args::get_locations;
use crate::cli::args::{
    Commands, IdentificationCommands, ObservationCommands, PlaceCommands, ProjectCommands,
    StatsCommands, TaxonCommands, UserCommands,
};
use crate::client::INaturalistClient;
use crate::error::ClientError;
use crate::models::{
    IdentificationSearchParams, ObservationSearchParams, PlaceSearchParams, ProjectSearchParams,
    SearchParams, execute_identifier_stats, execute_location_stats, execute_observer_stats,
    execute_species_stats,
};
use crate::utils::output::write_output;
use std::path::Path;

pub async fn execute_command(
    client: &INaturalistClient,
    command: Commands,
    format: &str,
    output: Option<&Path>,
) -> Result<(), ClientError> {
    match command {
        Commands::Stats(cmd) => execute_stats_command(client, cmd).await,
        Commands::Observations(cmd) => {
            execute_observation_command(client, cmd, format, output).await
        }
        Commands::Identifications(cmd) => {
            execute_identification_command(client, cmd, format, output).await
        }
        Commands::Taxa(cmd) => execute_taxon_command(client, cmd, format, output).await,
        Commands::Places(cmd) => execute_place_command(client, cmd, format, output).await,
        Commands::Projects(cmd) => execute_project_command(client, cmd, format, output).await,
        Commands::Users(cmd) => execute_user_command(client, cmd, format, output).await,
        Commands::Search(cmd) => execute_search_command(client, cmd, format, output).await,
        Commands::ControlledTerms(cmd) => {
            execute_controlled_terms_command(client, cmd, format, output).await
        }
    }
}

async fn execute_stats_command(
    client: &INaturalistClient,
    command: StatsCommands,
) -> Result<(), ClientError> {
    match command {
        StatsCommands::LocationStats(cmd) => {
            let resolved_locations = get_locations(&cmd.locations, &cmd.csv_file, &cmd.csv_column)?;
            execute_location_stats(
                client,
                resolved_locations,
                cmd.max_workers,
                cmd.params,
                cmd.output,
            )
            .await
        }
        StatsCommands::ObserverStats(cmd) => {
            let resolved_locations = get_locations(&cmd.locations, &cmd.csv_file, &cmd.csv_column)?;
            execute_observer_stats(
                client,
                resolved_locations,
                cmd.max_workers,
                cmd.params,
                cmd.output,
            )
            .await
        }
        StatsCommands::IdentifierStats(cmd) => {
            let resolved_locations = get_locations(&cmd.locations, &cmd.csv_file, &cmd.csv_column)?;
            execute_identifier_stats(
                client,
                resolved_locations,
                cmd.max_workers,
                cmd.params,
                cmd.output,
            )
            .await
        }
        StatsCommands::SpeciesStats(cmd) => {
            let resolved_locations = get_locations(&cmd.locations, &cmd.csv_file, &cmd.csv_column)?;
            execute_species_stats(
                client,
                resolved_locations,
                cmd.max_workers,
                cmd.params,
                cmd.output,
            )
            .await
        }
    }
}

async fn execute_observation_command(
    client: &INaturalistClient,
    command: ObservationCommands,
    format: &str,
    output: Option<&Path>,
) -> Result<(), ClientError> {
    match command {
        ObservationCommands::Search(args) => {
            let params = ObservationSearchParams {
                taxon_id: args.taxon_id,
                place_id: args.place_id,
                user_id: args.user_id,
                quality_grade: args.quality_grade,
                observed_on: args.observed_on,
                photos: Some(args.photos),
                sounds: Some(args.sounds),
                geo: Some(args.geo),
                verifiable: Some(args.verifiable),
                lat: args.lat,
                lng: args.lng,
                radius: args.radius,
                per_page: Some(args.per_page),
                ..Default::default()
            };
            let results = client.search_observations(&params).await?;
            write_output(&results, format.into(), output)?;
            Ok(())
        }
        ObservationCommands::Get(args) => {
            let result = client.get_observation(args.id).await?;
            write_output(&[result], format.into(), output)?;
            Ok(())
        }
    }
}

async fn execute_identification_command(
    client: &INaturalistClient,
    command: IdentificationCommands,
    format: &str,
    output: Option<&Path>,
) -> Result<(), ClientError> {
    match command {
        IdentificationCommands::Search(args) => {
            let params = IdentificationSearchParams {
                observation_id: args.observation_id,
                taxon_id: args.taxon_id,
                user_id: args.user_id,
                current: Some(args.current),
                per_page: Some(args.per_page),
                ..Default::default()
            };
            let results = client.search_identifications(&params).await?;
            write_output(&results, format.into(), output)?;
            Ok(())
        }
        IdentificationCommands::Get(args) => {
            let result = client.get_identification(args.id).await?;
            write_output(&[result], format.into(), output)?;
            Ok(())
        }
    }
}

async fn execute_taxon_command(
    client: &INaturalistClient,
    command: TaxonCommands,
    format: &str,
    output: Option<&Path>,
) -> Result<(), ClientError> {
    match command {
        TaxonCommands::Search(args) => {
            let results = client
                .search_taxa(
                    args.q.as_deref(),
                    args.taxon_id,
                    args.rank,
                    Some(args.per_page),
                )
                .await?;
            write_output(&results, format.into(), output)?;
            Ok(())
        }
        TaxonCommands::Get(args) => {
            let result = client.get_taxon(args.id).await?;
            write_output(&[result], format.into(), output)?;
            Ok(())
        }
        TaxonCommands::Autocomplete(args) => {
            let results = client
                .taxa_autocomplete(&args.query, Some(args.per_page))
                .await?;
            write_output(&results, format.into(), output)?;
            Ok(())
        }
        TaxonCommands::Suggest(args) => {
            let results = client
                .suggest_taxa(args.image_url.as_deref(), args.lat, args.lng)
                .await?;
            write_output(&results, format.into(), output)?;
            Ok(())
        }
    }
}

async fn execute_place_command(
    client: &INaturalistClient,
    command: PlaceCommands,
    format: &str,
    output: Option<&Path>,
) -> Result<(), ClientError> {
    match command {
        PlaceCommands::Search(args) => {
            let params = PlaceSearchParams {
                q: args.q,
                place_type: args.place_type,
                ancestor_place_id: args.ancestor_place_id,
                per_page: Some(args.per_page),
                ..Default::default()
            };
            let results = client.search_places(&params).await?;
            write_output(&results, format.into(), output)?;
            Ok(())
        }
        PlaceCommands::Get(args) => {
            let result = client.get_place(args.id).await?;
            write_output(&[result], format.into(), output)?;
            Ok(())
        }
        PlaceCommands::Nearby(args) => {
            let results = client
                .places_nearby(
                    args.nelat,
                    args.nelng,
                    args.swlat,
                    args.swlng,
                    args.name.as_deref(),
                )
                .await?;
            write_output(&results, format.into(), output)?;
            Ok(())
        }
        PlaceCommands::Autocomplete(args) => {
            let results = client
                .places_autocomplete(&args.query, Some(args.per_page))
                .await?;
            write_output(&results, format.into(), output)?;
            Ok(())
        }
    }
}

async fn execute_project_command(
    client: &INaturalistClient,
    command: ProjectCommands,
    format: &str,
    output: Option<&Path>,
) -> Result<(), ClientError> {
    match command {
        ProjectCommands::Search(args) => {
            let params = ProjectSearchParams {
                q: args.q,
                place_id: args.place_id,
                taxon_id: args.taxon_id,
                user_id: args.user_id,
                per_page: Some(args.per_page),
                ..Default::default()
            };
            let results = client.search_projects(&params).await?;
            write_output(&results, format.into(), output)?;
            Ok(())
        }
        ProjectCommands::Get(args) => {
            let result = client.get_project(args.id).await?;
            write_output(&[result], format.into(), output)?;
            Ok(())
        }
        ProjectCommands::Members(args) => {
            let results = client
                .get_project_members(args.id, None, Some(args.per_page))
                .await?;
            write_output(&results, format.into(), output)?;
            Ok(())
        }
    }
}

async fn execute_user_command(
    client: &INaturalistClient,
    command: UserCommands,
    format: &str,
    output: Option<&Path>,
) -> Result<(), ClientError> {
    match command {
        UserCommands::Get(args) => {
            let result = client.get_user(args.id).await?;
            write_output(&[result], format.into(), output)?;
            Ok(())
        }
        UserCommands::Me => {
            let result = client.get_current_user().await?;
            write_output(&[result], format.into(), output)?;
            Ok(())
        }
        UserCommands::Autocomplete(args) => {
            let results = client
                .users_autocomplete(&args.query, Some(args.per_page))
                .await?;
            write_output(&results, format.into(), output)?;
            Ok(())
        }
    }
}

async fn execute_search_command(
    client: &INaturalistClient,
    cmd: crate::cli::args::SearchCommand,
    format: &str,
    output: Option<&Path>,
) -> Result<(), ClientError> {
    let params = SearchParams {
        q: cmd.query,
        sources: cmd.sources,
        per_page: Some(cmd.per_page),
        ..Default::default()
    };
    let results = client.search(&params).await?;
    write_output(&results, format.into(), output)?;
    Ok(())
}

async fn execute_controlled_terms_command(
    client: &INaturalistClient,
    cmd: crate::cli::args::ControlledTermsCommand,
    format: &str,
    output: Option<&Path>,
) -> Result<(), ClientError> {
    let results = client.get_controlled_terms(cmd.taxon_id).await?;
    write_output(&results, format.into(), output)?;
    Ok(())
}
