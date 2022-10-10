use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(
        short,
        long,
        env,
        value_delimiter = ',',
        help = "comma separated values are allowed"
    )]
    multi: Vec<String>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args.multi);
}
