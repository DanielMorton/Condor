use serde::Serialize;
use csv::Writer;
use std::io::{self, Write};
use std::path::Path;

use crate::error::Result;

pub enum OutputFormat {
    Csv,
    Json,
    Pretty,
}

impl From<&str> for OutputFormat {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "json" => OutputFormat::Json,
            "pretty" => OutputFormat::Pretty,
            _ => OutputFormat::Csv,
        }
    }
}

pub fn write_output<T: Serialize>(
    items: &[T],
    format: OutputFormat,
    path: Option<&Path>,
) -> Result<()> {
    match format {
        OutputFormat::Csv => write_csv(items, path)?,
        OutputFormat::Json => write_json(items, path, false)?,
        OutputFormat::Pretty => write_json(items, path, true)?,
    }
    Ok(())
}

fn write_csv<T: Serialize>(
    items: &[T],
    path: Option<&Path>,
) -> Result<()> {
    let mut writer: Box<dyn Write> = match path {
        Some(p) => Box::new(std::fs::File::create(p)?),
        None => Box::new(io::stdout()),
    };

    let mut csv_writer = Writer::from_writer(writer);
    for item in items {
        csv_writer.serialize(item)?;
    }
    csv_writer.flush()?;
    Ok(())
}

fn write_json<T: Serialize>(
    items: &[T],
    path: Option<&Path>,
    pretty: bool,
) -> Result<()> {
    let output = if pretty {
        serde_json::to_string_pretty(items)?
    } else {
        serde_json::to_string(items)?
    };

    match path {
        Some(p) => std::fs::write(p, output)?,
        None => println!("{}", output),
    }
    Ok(())
}
