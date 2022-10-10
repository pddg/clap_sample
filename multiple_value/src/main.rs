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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_arg() {
        let args = Args::try_parse_from([""]);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().multi.len(), 0);
    }

    #[test]
    fn one_arg() {
        let args = Args::try_parse_from(["", "--multi", "one"]);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().multi, vec!["one"]);
    }

    #[test]
    fn two_arg() {
        let args = Args::try_parse_from(["", "--multi", "one", "--multi", "two"]);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().multi, vec!["one", "two"]);
    }

    #[test]
    fn two_arg_with_comma() {
        let args = Args::try_parse_from(["", "--multi", "one,two"]);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().multi, vec!["one", "two"]);
    }

    #[test]
    fn mixed() {
        let args = Args::try_parse_from(["", "--multi", "one,two", "--multi", "three"]);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().multi, vec!["one", "two", "three"]);
    }
}
