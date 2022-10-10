use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, env, help = "Value from env")]
    from_env: String,

    #[arg(short, long, env, hide_env_values = true, help = "Any credential")]
    credential: String,

    #[arg(
        short = 'n',
        long,
        env,
        hide_env = true,
        help = "This can be specified by env var"
    )]
    hidden_env: String,
}

fn main() {
    let args = Args::parse();
    println!("from_env: {}", args.from_env);
    println!("credential: {}", args.credential);
    println!("hidden: {}", args.hidden_env);
}
