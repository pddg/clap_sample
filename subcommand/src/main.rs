use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long, global = true, help = "Global flag")]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
enum Commands {
    #[command(about = "help for hoge")]
    Hoge {
        #[arg(short, long)]
        opt: String,
    },
    #[command(about = "help for fuga")]
    Fuga(FugaArgs),
}

#[derive(Debug, Args, PartialEq, Eq)]
struct FugaArgs {
    #[arg(short, long)]
    opt: String,

    #[arg(short, long, global = true, help = "global option for fuga")]
    global: bool,

    #[command(subcommand)]
    command: Option<SubCommands>,
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
enum SubCommands {
    #[command(about = "help for nested")]
    Nested {
        #[arg(short, long, help = "opt for nested")]
        opt: String,
    },
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
                Some(c) => match c {
                    SubCommands::Nested { opt } => {
                        println!("nested {}", opt);
                    }
                },
                None => {
                    println!("sub command is not specified");
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_arg() {
        let args = Cli::try_parse_from([""]);
        assert!(args.is_err());
        let args = Cli::try_parse_from(["", "hoge"]);
        assert!(args.is_err());
        let args = Cli::try_parse_from(["", "fuga"]);
        assert!(args.is_err());
        let args = Cli::try_parse_from(["", "fuga", "nested"]);
        assert!(args.is_err());
    }

    #[test]
    fn hoge() {
        let args = Cli::try_parse_from(["", "hoge", "--opt", "1"]);
        assert!(args.is_ok());
        assert_eq!(
            args.unwrap().command,
            Commands::Hoge {
                opt: "1".to_string()
            }
        );
    }

    #[test]
    fn fuga() {
        let args = Cli::try_parse_from(["", "fuga", "--opt", "1"]);
        assert!(args.is_ok());
        assert_eq!(
            args.unwrap().command,
            Commands::Fuga(FugaArgs {
                opt: "1".to_string(),
                command: None,
                global: false,
            })
        );
    }

    #[test]
    fn fuga_nested() {
        let args = Cli::try_parse_from(["", "fuga", "--opt", "1", "nested", "--opt", "1"]);
        assert!(args.is_ok());
        assert_eq!(
            args.unwrap().command,
            Commands::Fuga(FugaArgs {
                opt: "1".to_string(),
                command: Some(SubCommands::Nested {
                    opt: "1".to_string(),
                }),
                global: false,
            })
        );
    }

    #[test]
    fn verbose_can_specify_all_sub_commands() {
        let args = Cli::try_parse_from(["", "--verbose", "hoge", "--opt", "1"]);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().verbose, true);
        let args = Cli::try_parse_from(["", "hoge", "--verbose", "--opt", "1"]);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().verbose, true);
        let args = Cli::try_parse_from(["", "fuga", "--verbose", "--opt", "1"]);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().verbose, true);
        let args = Cli::try_parse_from([
            "",
            "fuga",
            "--opt",
            "1",
            "nested",
            "--verbose",
            "--opt",
            "1",
        ]);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().verbose, true);
    }

    #[test]
    fn globa_can_specify_from_fuga_or_its_sub_commands() {
        let args = Cli::try_parse_from(["", "fuga", "--global", "--opt", "1"]);
        assert!(args.is_ok());
        let args =
            Cli::try_parse_from(["", "fuga", "--opt", "1", "nested", "--global", "--opt", "1"]);
        assert!(args.is_ok());
        let args = Cli::try_parse_from(["", "hoge", "--global", "--opt", "1"]);
        assert!(args.is_err());
        let args = Cli::try_parse_from(["", "--global"]);
        assert!(args.is_err());
    }
}
