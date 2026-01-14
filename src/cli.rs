use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ps tools")]
#[command(about = "백준 문제 불러오기", long_about = None)]
pub(crate) struct Cli {
    pub num: Option<u32>,

    #[command(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    T, //try
    S, //submit
    L { 
        #[arg(short, long)]
        id: String,
        #[arg(short, long)]
        password: String,
    }, //login
    Q, //quit
}
