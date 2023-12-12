use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "timr")]
#[command(about = "timing tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = false)]
    List {},

    #[command(arg_required_else_help = true)]
    Add { name: String, description: String },

    #[command(arg_required_else_help = true)]
    Start { id: u32 },

    #[command(arg_required_else_help = true)]
    Complete { id: u32 },

    #[command(arg_required_else_help = true)]
    Delete { id: u32 },
}
