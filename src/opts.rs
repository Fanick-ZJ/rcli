use core::fmt;
use std::str::FromStr;

use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(name = "rcli", version, author = "Rust CLI")]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    // name 是子命令的名字，about是子命令的描述
    #[command(name = "csv", about = "Convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(
        short,
        long,
        long_help = "Input file path",
        value_parser=value_input_file
    )]
    pub input: String,
    // default_value 与 default_value_t 的区别在于default_value后的参数如果能够Into<T>的话，那么他就会自动转化，而default_value_t不会
    #[arg(
        short,
        long,
        long_help = "Output file path"
    )]
    pub output: Option<String>,
    #[arg(long, long_help = "Output format", value_parser=parse_format, default_value = "json" )]
    pub format: OutputFormat,
    #[arg(short, long, long_help = "Delimiter", default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, long_help = "CSV has Header", default_value_t = true)]
    pub header: bool,
}

fn value_input_file(file: &str) -> Result<String, String> {
    if !std::path::Path::new(&file).exists() {
        return Err("Input file does not exist".to_string());
    }
    if file.ends_with(".csv") {
        Ok(file.to_string())
    } else {
        Err("Input file must be a CSV file".to_string())
    }
}


fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse::<OutputFormat>()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml"
        } 
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error; 

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            v => Err(anyhow::anyhow!(format!("Invalid output format: {}", v))), 
        } 
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    } 
}