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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_arg() {
        let args = Args::try_parse_from([""]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert!(!args.verbose);
        assert!(args.non_verbose);
    }

    #[test]
    fn with_verbose() {
        let args = Args::try_parse_from(["", "--verbose"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert!(args.verbose);
        assert!(args.non_verbose);
    }

    #[test]
    fn with_non_verbose() {
        let args = Args::try_parse_from(["", "--non-verbose"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert!(!args.verbose);
        assert!(!args.non_verbose);
    }

    #[test]
    fn specify_both() {
        let args = Args::try_parse_from(["", "--verbose", "--non-verbose"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert!(args.verbose);
        assert!(!args.non_verbose);
    }

    #[test]
    fn verbose_takes_no_arg() {
        let args = Args::try_parse_from(["", "--verbose", "value"]);
        assert!(args.is_err());
    }

    #[test]
    fn non_verbose_takes_no_arg() {
        let args = Args::try_parse_from(["", "--non-verbose", "value"]);
        assert!(args.is_err());
    }
}
