use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::opts::OutputFormat;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String,  format: &OutputFormat) -> Result<()> {
    let mut rdr = Reader::from_path(input)?;
    let mut records = Vec::new();
    let headers = rdr.headers()?.clone();
    for result in rdr.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        records.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&records)?,
        OutputFormat::Yaml => serde_yaml::to_string(&records)?,
    };
    std::fs::write(&output, &content)?;
    Ok(())
}
