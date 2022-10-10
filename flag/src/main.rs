use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long, action = clap::ArgAction::SetFalse)]
    non_verbose: bool,
}

fn main() {
    let args = Args::parse();
    println!("verbose: {}", args.verbose);
    println!("non-verbose: {}", args.non_verbose);
}
