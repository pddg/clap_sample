use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[command(group(ArgGroup::new("how_to_input").required(true).args(["input", "input_from_stdin"])))]
struct Args {
    #[arg(long, help = "input from arg")]
    input: Option<String>,

    #[arg(long, help = "input from stdin")]
    input_from_stdin: bool,
}

fn main() {
    let args = Args::parse();
    println!(
        "input: {}, input_from_stdin: {}",
        args.input.unwrap_or("not specified".to_string()),
        args.input_from_stdin
    );
}
