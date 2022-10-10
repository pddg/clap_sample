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

// これらのテストは環境変数に依存するため、1スレッドで直列に実行されなければいけません。
#[cfg(test)]
mod test {
    use super::*;
    use std::env;

    #[test]
    fn no_arg_no_env() {
        env::remove_var("FROM_ENV");
        env::remove_var("CREDENTIAL");
        env::remove_var("HIDDEN_ENV");
        let args = Args::try_parse_from([""]);
        assert!(args.is_err());
    }

    #[test]
    fn with_arg_no_env() {
        env::remove_var("FROM_ENV");
        env::remove_var("CREDENTIAL");
        env::remove_var("HIDDEN_ENV");
        let args = Args::try_parse_from([
            "",
            "--from-env",
            "value",
            "--credential",
            "password",
            "--hidden-env",
            "hidden",
        ]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.from_env, "value");
        assert_eq!(args.credential, "password");
        assert_eq!(args.hidden_env, "hidden");
    }

    #[test]
    fn no_arg_with_env() {
        env::set_var("FROM_ENV", "value");
        env::set_var("CREDENTIAL", "password");
        env::set_var("HIDDEN_ENV", "hidden");
        let args = Args::try_parse_from([""]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.from_env, "value");
        assert_eq!(args.credential, "password");
        assert_eq!(args.hidden_env, "hidden");
    }
}
