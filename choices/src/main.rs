use clap::{Parser, ValueEnum};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, value_enum, default_value_t = LogLevel::Info, help = "Log level")]
    log_level: LogLevel,

    #[arg(long, value_enum, value_delimiter = ',', help = "Fruits")]
    fruits: Vec<Fruit>,
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Critical,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let raw = format!("{:?}", self);
        write!(f, "{}", raw.to_uppercase())
    }
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
enum Fruit {
    Apple,
    Orange,
    Banana,
}

impl Display for Fruit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    let args = Args::parse();
    println!("log_level: {}", args.log_level);
    println!("fruits: {:?}", args.fruits);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_args() {
        let args = Args::try_parse_from([""]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.log_level, LogLevel::Info);
        assert_eq!(args.fruits, vec![]);
    }

    #[test]
    fn log_level_debug() {
        let args = Args::try_parse_from(["", "--log-level", "debug"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.log_level, LogLevel::Debug);
        assert_eq!(args.fruits, vec![]);
    }

    #[test]
    fn fruit_one() {
        let args = Args::try_parse_from(["", "--fruits", "apple"]);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().fruits, vec![Fruit::Apple]);
    }

    #[test]
    fn fruit_multi() {
        let args = Args::try_parse_from(["", "--fruits", "apple,orange"]);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().fruits, vec![Fruit::Apple, Fruit::Orange]);

        let args = Args::try_parse_from(["", "--fruits", "apple", "--fruits", "orange"]);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().fruits, vec![Fruit::Apple, Fruit::Orange]);
    }

    #[test]
    fn log_level_multi() {
        let args = Args::try_parse_from(["", "--log-level", "info", "--log-level", "error"]);
        assert!(args.is_err());
    }

    #[test]
    fn fruit_and_loglevel() {
        let args = Args::try_parse_from(["", "--log-level", "debug", "--fruits", "orange,apple"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.log_level, LogLevel::Debug);
        assert_eq!(args.fruits, vec![Fruit::Orange, Fruit::Apple]);
    }

    #[test]
    fn invalid_log_level() {
        let args = Args::try_parse_from(["", "--log-level", "invalid"]);
        assert!(args.is_err());

        let args = Args::try_parse_from(["", "--log-level", ""]);
        assert!(args.is_err());
    }

    #[test]
    fn invalid_fruit() {
        let args = Args::try_parse_from(["", "--fruits", "invalid"]);
        assert!(args.is_err());

        let args = Args::try_parse_from(["", "--fruits", "invalid,apple"]);
        assert!(args.is_err());

        let args = Args::try_parse_from(["", "--fruits", ""]);
        assert!(args.is_err());
    }

    #[test]
    fn unknown_option() {
        let args = Args::try_parse_from(["", "invalid"]);
        assert!(args.is_err());

        let args = Args::try_parse_from(["", "--invalid", "invalid"]);
        assert!(args.is_err());

        let args = Args::try_parse_from(["", "--invalid"]);
        assert!(args.is_err());
    }
}