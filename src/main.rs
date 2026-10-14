use clap::{Args, Parser, Subcommand};

mod errors;
mod infra;
mod models;
mod repos;
mod services;
mod app;

use crate::infra::database;
use crate::services::task_service;
use crate::app::app::app;
use rusqlite::{Connection, Result};
use std::io;


#[derive(Parser)]
#[command(name = "task_list")]
#[command(version = "0.0.1")]
#[command(about = "Gerenciador de tarefas")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(clap::Subcommand, Clone)]
enum Commands {
    Add { description: String },
    List,
    Check { id: String },
    Delete { id: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();
    let conn = Connection::open("data.db")?;
    infra::database::init_database(&conn);

    app(cli, &conn);
    Ok(())
}

