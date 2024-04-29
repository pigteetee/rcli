use std::path::Path;

use clap::Parser;

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

use std::fs;

use anyhow::Result;

pub fn action(input: &str, output: &str) -> Result<()> {
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
