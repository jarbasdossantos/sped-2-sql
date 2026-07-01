use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "sped-2-sql",
    version,
    about = "Import and export SPED files to/from a SQL database",
    long_about = None
)]
pub struct Cli {
    /// Database URL (e.g. SQLite file path). Overrides DATABASE_URL env var.
    #[arg(long, env = "DATABASE_URL")]
    pub database_url: Option<String>,

    /// Use in-memory database
    #[arg(long, default_value_t = false)]
    pub memory_db: bool,

    /// Log level: error, warn, info, debug, trace
    #[arg(long, env = "RUST_LOG", default_value = "info")]
    pub log_level: String,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Import one or more SPED files into the database
    Import {
        /// SPED files to import (.txt)
        #[arg(required = true)]
        files: Vec<PathBuf>,

        /// SPED type
        #[arg(long, value_enum, default_value_t = SpedTypeArg::Efd)]
        sped_type: SpedTypeArg,

        /// Comma-separated list of registers to import (e.g. "0000,0001"). If omitted, import all.
        #[arg(long)]
        registers: Option<String>,
    },

    /// Export a previously imported file from the database
    Export {
        /// File id to export
        #[arg(long)]
        file_id: i32,

        /// SPED type
        #[arg(long, value_enum, default_value_t = SpedTypeArg::Efd)]
        sped_type: SpedTypeArg,

        /// Comma-separated list of registers to export. If omitted, export all.
        #[arg(long)]
        registers: Option<String>,

        /// Output file path. If omitted, write to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },

    /// Run pending database migrations
    Migrate,

    /// Validate a SPED file without importing
    Validate {
        /// SPED file to validate
        #[arg(required = true)]
        file: PathBuf,

        /// SPED type
        #[arg(long, value_enum, default_value_t = SpedTypeArg::Efd)]
        sped_type: SpedTypeArg,
    },
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum SpedTypeArg {
    Efd,
    IcmsIpi,
}