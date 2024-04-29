use std::fs;
use std::path::Path;
use std::str::FromStr;

use anyhow::Result;

use clap::Parser;

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct Command {
    pub file: Option<String>,

    #[arg(
        short,
        long,
        help = "input file path",
        value_parser = validate_input_file_path
    )]
    pub input: String,

    #[arg(short, long, help = "output file path", default_value = "output.json")]
    pub output: String,

    #[arg(long, help = "output format", default_value = "json", value_parser = parse_input_format)]
    pub format: OutputFormat,

    #[arg(short, long, help = "delimiter", default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, help = "csv has header or not", default_value_t = true)]
    pub header: bool,
}

fn validate_input_file_path(input: &str) -> Result<String, String> {
    if Path::new(input).exists() {
        Ok(input.into())
    } else {
        Err("file not found".into())
    }
}

fn parse_input_format(format: &str) -> Result<OutputFormat> {
    format.parse()
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => anyhow::bail!("Invalid format"),
        }
    }
}

pub fn action(input: &str, output: &str, options: &Command) -> Result<()> {
    println!("format: {:?}", options.format);

    let mut reader = csv::Reader::from_path(input)?;

    let mut result: Vec<_> = Vec::with_capacity(1024);

    let headers = reader.headers()?.clone();

    for r in reader.records() {
        let record = r?;

        let rec = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();

        result.push(rec);
    }

    let result_json = serde_json::to_string_pretty(&result)?;

    fs::write(output, result_json)?;

    return Ok(());
}
