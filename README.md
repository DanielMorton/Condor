# iNaturalist API Client

A comprehensive Rust CLI tool for interacting with the iNaturalist API. Supports both predefined endpoints and arbitrary GET requests to any iNaturalist API endpoint, with rate limiting, retry logic, and parallel batch processing.

## Features

- **Generic GET Requests**: Query any iNaturalist API endpoint
- **Batch Processing**: Process multiple locations/endpoints in parallel
- **Rate Limiting**: Configurable request rate limiting
- **Retry Logic**: Automatic retry with exponential backoff on rate limits
- **Multiple Output Formats**: JSON (pretty or compact), CSV
- **Predefined Endpoints**: Convenient access to common endpoints (observations, taxa, places, etc.)

## Installation

```bash
# Clone the repository
git clone https://github.com/DanielMorton/Condor.git
cd Condor

# Build the project
cargo build --release

# The binary will be at target/release/inaturalist-cli
```

## Usage

### Basic Structure

```bash
inaturalist-cli [OPTIONS] <COMMAND>
```

### Global Options

- `-r, --rate-limit <RATE_LIMIT>`: API requests per second (default: 1.0)
- `-v, --verbose`: Enable verbose logging
- `--format <FORMAT>`: Output format - `table` (default), `json`, or `csv`
- `-h, --help`: Print help
- `-V, --version`: Print version

### Commands

#### Generic GET Request

Make a GET request to any iNaturalist API endpoint:

```bash
# Single GET request
inaturalist-cli get --path "places/1"

# With query parameters
inaturalist-cli get --path "observations" --param "place_id=1" --param "per_page=10"

# With CSV output
inaturalist-cli --format csv get --path "places/autocomplete" --param "q=San Francisco"
```

#### Batch GET Requests

Process multiple endpoints in parallel:

```bash
# Batch with same parameters for all paths
inaturalist-cli batch-get --paths "places/1,places/2,places/3" --max-workers 4

# Batch with individual parameters per path
inaturalist-cli batch-get-with-params --path-params "observations?place_id=1&per_page=5,taxa/123?per_page=3" --max-workers 2
```

#### Predefined Endpoint Commands

```bash
# Get observation histogram
inaturalist-cli histogram --location "1" --interval "month_of_year"

# Get observer statistics
inaturalist-cli observers --location "1" --per-page 20

# Get identifier statistics  
aturalist-cli identifiers --location "1" --per-page 20

# Get species counts
inaturalist-cli species-counts --location "1" --per-page 20

# Search taxa
inaturalist-cli taxa-search --query "Quercus" --rank "species" --per-page 10

# Autocomplete taxa
inaturalist-cli taxa-autocomplete --query "Quercus" --per-page 5

# Search places
inaturalist-cli places-search --query "San Francisco"

# Autocomplete places
inaturalist-cli places-autocomplete --query "San"

# Find nearby places
inaturalist-cli places-nearby --nelat 37.8 --nelng -122.3 --swlat 37.7 --swlng -122.4

# Search projects
inaturalist-cli projects-search --query "butterfly" --per-page 10
```

#### Batch Processing for Predefined Endpoints

```bash
# Process multiple locations for histogram
inaturalist-cli batch --locations "1,2,3" --stat-type histogram --max-workers 4

# Process multiple locations for species counts
inaturalist-cli batch --locations "1,2,3" --stat-type species --max-workers 4
```

## Output Formats

### JSON (default for table format)

Pretty-printed JSON output:

```bash
inaturalist-cli get --path "places/1"
```

Output:
```json
{
  "total_results": 1,
  "page": 1,
  "per_page": 1,
  "results": [
    {
      "id": 1,
      "name": "United States",
      "display_name": "United States",
      ...
    }
  ]
}
```

### Compact JSON

```bash
inaturalist-cli --format json get --path "places/1"
```

### CSV

Flattened CSV with dot-notation for nested fields:

```bash
inaturalist-cli --format csv get --path "places/autocomplete" --param "q=San Francisco"
```

Output:
```csv
admin_level,ancestor_place_ids,bbox_area,display_name,id,name,place_type
20,"97394,6793,8295,58162",0.002335866060708414,"San Francisco Huehuetl\u00e1n, OA, MX",58162,"San Francisco Huehuetl\u00e1n",1000
20,"97394,6793,8295,58159",0.011794068305732947,"San Francisco Chapulapa, OA, MX",58159,"San Francisco Chapulapa",1000
...
```

**CSV Notes:**
- Nested objects use dot notation (e.g., `bounding_box_geojson.coordinates`)
- Simple arrays (≤5 primitive values) are expanded as comma-separated values
- Complex arrays are stored as JSON strings in the CSV cell
- All fields from the API response are included as columns

## Examples

### Example 1: Get recent observations for a place

```bash
inaturalist-cli --format csv get --path "observations" \
  --param "place_id=1" \
  --param "per_page=50" \
  --param "order=desc" \
  --param "order_by=created_at"
```

### Example 2: Batch process multiple locations

```bash
inaturalist-cli batch-get \
  --paths "observations?place_id=1&per_page=10,observations?place_id=2&per_page=10" \
  --max-workers 2 \
  --format csv
```

### Example 3: Search for taxa and save as CSV

```bash
inaturalist-cli --format csv taxa-search --query "Quercus" --per-page 50 > oak_species.csv
```

### Example 4: Get species counts for multiple locations in parallel

```bash
inaturalist-cli batch --locations "1,2,3,4,5" --stat-type species --max-workers 5
```

## Rate Limiting

The client respects iNaturalist's rate limits with automatic retry:

- Default: 1 request per second
- Adjust with `--rate-limit` flag (e.g., `--rate-limit 0.5` for 0.5 req/s)
- Automatic exponential backoff on 429 responses

## Endpoints

### Predefined Endpoints

| Command | Endpoint | Description |
|---------|----------|-------------|
| `histogram` | `observations/histogram` | Observation count histogram |
| `observers` | `observations/observers` | Top observers for a location |
| `identifiers` | `observations/identifiers` | Top identifiers for a location |
| `species-counts` | `observations/species_counts` | Species counts for a location |
| `taxa-search` | `taxa` | Search taxa |
| `taxa-autocomplete` | `taxa/autocomplete` | Autocomplete taxa names |
| `places-search` | `places` | Search places |
| `places-autocomplete` | `places/autocomplete` | Autocomplete place names |
| `places-nearby` | `places/nearby` | Find places near coordinates |
| `projects-search` | `projects` | Search projects |

### Generic GET

Any iNaturalist API endpoint can be queried using the `get` command:

```bash
# Any valid iNaturalist API path works
inaturalist-cli get --path "users/1"
aturalist-cli get --path "projects/123/observations"
aturalist-cli get --path "identifications" --param "user_id=1" --param "per_page=10"
```

## API Documentation

For complete iNaturalist API documentation, see: [https://api.inaturalist.org/v1/docs/](https://api.inaturalist.org/v1/docs/)

## Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License.
