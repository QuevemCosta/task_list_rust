use clap::Parser;
use rusqlite::{Connection, Result};

mod app;
mod errors;
mod infra;
mod models;
mod repos;
mod services;

use crate::app::app::app;
use crate::infra::database;

#[derive(Parser)]
#[command(name = "task_list")]
#[command(version = "0.0.1")]
#[command(about = "Gerenciador de tarefas")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Clone)]
pub enum Commands {
    Add { description: String },
    List,
    Check { id: String },
    Delete { id: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let conn = Connection::open("data.db")?;
    database::init_database(&conn);

    app(cli, &conn);
    Ok(())
}
