//! Defines the CLI for `cargo-spdx`.

use anyhow::{anyhow, Error, Result};
use clap::Parser;
use std::fs::File;
use std::io::{stdout, BufWriter, Write};
use std::path::PathBuf;
use std::str::FromStr;

/// Contains the parsed CLI arguments.
#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct Cli {
    /// The output format to use.
    #[clap(short, long)]
    pub fmt: Option<Format>,

    /// The URL where the SBOM will be hosted.
    #[clap(short = 'H', long)]
    pub host_url: String,

    /// The name of a file to write out to.
    #[clap(short, long)]
    pub output: Option<PathBuf>,

    /// Ignored.
    ///
    /// This is the "spdx" part when called as a Cargo subcommand.
    _spdx: String,
}

impl Cli {
    /// Get a writer to the correct output stream.
    pub fn output_writer(&self) -> Result<Box<dyn Write>> {
        if let Some(file_path) = &self.output {
            if file_path.exists() {
                return Err(anyhow!("'{}' already exists", file_path.display()));
            }

            Ok(Box::new(BufWriter::new(File::create(file_path)?)))
        } else {
            Ok(Box::new(BufWriter::new(stdout())))
        }
    }
}

/// The output format for the SPDX document.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Format {
    /// Key-value format.
    KeyValue,

    /// JSON format.
    Json,

    /// YAML format.
    Yaml,

    /// RDF format.
    Rdf,
}

impl Default for Format {
    fn default() -> Self {
        Format::KeyValue
    }
}

impl FromStr for Format {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "kv" => Ok(Format::KeyValue),
            "json" => Ok(Format::Json),
            "yaml" => Ok(Format::Yaml),
            "rdf" => Ok(Format::Rdf),
            s => Err(anyhow!("unknown format '{}'", s)),
        }
    }
}
