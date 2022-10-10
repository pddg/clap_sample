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
    println!("opt: {}", args.opt.unwrap_or_else(|| "not specified".to_string()));
    println!("inputs: {:?}", args.inputs);
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
    fn with_required() {
        let args = Args::try_parse_from(["", "-n", "name", "-c", "1"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.name, "name");
        assert_eq!(args.count, 1);
        assert_eq!(args.opt, None);
        assert_eq!(args.inputs.len(), 0);
    }

    #[test]
    fn with_non_required() {
        let args = Args::try_parse_from(["", "-n", "name", "-c", "1", "-o", "opt", "-i", "one"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.name, "name");
        assert_eq!(args.count, 1);
        assert_eq!(args.opt, Some("opt".to_string()));
        assert_eq!(args.inputs, vec!["one"]);
    }

    #[test]
    fn non_integer() {
        let args = Args::try_parse_from(["", "-n", "name", "-c", "invalid"]);
        assert!(args.is_err());
    }
}
