use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};

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

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut rdr = Reader::from_path(input)?;
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: Player = result?;
        println!("{:?}", record);
        records.push(record);
    }
    let output_json = serde_json::to_string_pretty(&records)?;
    std::fs::write(&output_json, output)?;
    Ok(())
}
