use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short = 'n', long = "name", help = "your name")]
    name: String,

    #[arg(short, long, help = "a 32bit integer")]
    count: i32,

    #[arg(short, long, help = "optional value")]
    opt: Option<String>,

    #[arg(short, long, help = "multiple inputs")]
    inputs: Vec<String>,
}

fn main() {
    let args = Args::parse();
    println!("name: {}", args.name);
    println!("count: {}", args.count);
    println!("opt: {}", args.opt.unwrap_or("not specified".to_string()));
    println!("inputs: {:?}", args.inputs);
}
