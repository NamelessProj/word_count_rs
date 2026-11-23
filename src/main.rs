use clap::{Parser, ValueEnum};
use std::fs;

#[derive(Clone, ValueEnum)]
enum CountType {
    Word,
    Line,
    Character,
}

#[derive(Parser)]
struct Args {
    #[arg(long)]
    file: String,

    #[arg(long = "type", value_enum)]
    kind: CountType,
}

fn main() {
    let args = Args::parse();
    let text = fs::read_to_string(&args.file).expect("Failed to read file");

    let count = match args.kind {
        CountType::Word => text.split_whitespace().count(),
        CountType::Line => text.lines().count(),
        CountType::Character => text.chars().count(),
    };

    println!("{count}");
}
