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
        args.input.unwrap_or_else(|| "not specified".to_string()),
        args.input_from_stdin
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_arg() {
        let args = Args::try_parse_from([""]);
        assert!(args.is_err());
    }

    #[test]
    fn with_input() {
        let args = Args::try_parse_from(["", "--input", "value"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.input, Some("value".to_string()));
        assert!(!args.input_from_stdin);
    }

    #[test]
    fn with_input_from_stdin() {
        let args = Args::try_parse_from(["", "--input-from-stdin"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.input, None);
        assert!(args.input_from_stdin);
    }

    #[test]
    fn both_options_are_specified() {
        let args = Args::try_parse_from(["", "--input-from-stdin", "--input", "value"]);
        assert!(args.is_err());
    }

    #[test]
    fn input_with_no_arg() {
        let args = Args::try_parse_from(["", "--input"]);
        assert!(args.is_err());
    }

    #[test]
    fn input_from_stdin_with_arg() {
        let args = Args::try_parse_from(["", "--input-from-stdin", "value"]);
        assert!(args.is_err());
    }
}
