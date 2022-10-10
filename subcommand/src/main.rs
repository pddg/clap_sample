use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long, global = true, help = "Global flag")]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "help for hoge")]
    Hoge {
        #[arg(short, long)]
        opt: String,
    },
    #[command(about = "help for fuga")]
    Fuga(FugaArgs),
}

#[derive(Debug, Args)]
struct FugaArgs {
    #[arg(short, long)]
    opt: String,

    #[arg(short, long, global = true, help = "global option for fuga")]
    global: bool,

    #[command(subcommand)]
    command: Option<SubCommands>,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    #[command(about = "help for nested")]
    Nested {
        #[arg(short, long, help="opt for nested")]
        opt: String,
    }
}

fn main() {
    let cli = Cli::parse();
    println!("verbose: {}", cli.verbose);
    match cli.command {
        Commands::Hoge { opt } => {
            println!("hoge {}", opt);
        }
        Commands::Fuga(fuga) => {
            println!("fuga {}", fuga.opt);
            println!("global {}", fuga.global);
            match fuga.command {
                Some(c) => {
                    match c {
                        SubCommands::Nested { opt } => {
                            println!("nested {}", opt);
                        }
                    }
                }
                None => {
                    println!("sub command is not specified");
                }
            }
        }
    }
}