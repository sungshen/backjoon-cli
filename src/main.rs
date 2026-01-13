use clap::Parser;

#[derive(Parser)]
struct Cli {
    num: i32,
}

fn main() {
    let cli  = Cli::parse();
}

