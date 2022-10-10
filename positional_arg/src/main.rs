use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(value_name = "FILE", help = "位置引数の説明")]
    pos_arg: String,

    #[arg(last = true, help = "ここはパースされない")]
    last_args: Vec<String>,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.pos_arg);
    println!("last_args: {:?}", args.last_args);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_arg() {
        let args = Args::try_parse_from(["", "hoge"]);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().pos_arg, "hoge".to_string());
    }
}
