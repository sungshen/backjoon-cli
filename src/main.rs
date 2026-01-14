use crate::web::import::import;
use crate::cmd::option;
use crate::cli::Cli;
use clap::Parser;

mod cmd;
mod web;
pub(crate) mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli  = Cli::parse();
    match &cli.num {
        Some(x) => import(x),
        None =>  option(&cli.commands),
    }
}

