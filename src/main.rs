mod cli;
mod db;
mod interface;
mod model;

use crate::cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::List {} => {
            interface::list_tasks();
        }
        Commands::Add { name, description } => {
            interface::add_task(name, description);
        }
        Commands::Start { id } => {
            interface::start_task(id);
        }
        Commands::Complete { id } => {
            interface::complete_task(id);
        }
        Commands::Delete { id } => {
            interface::delete_task(id);
        }
    }
}
