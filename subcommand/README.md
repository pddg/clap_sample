# subcommand

```bash
❯ cargo run -q -- --help     
Usage: subcommand [OPTIONS] <COMMAND>

Commands:
  hoge  help for hoge
  fuga  help for fuga
  help  Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose  Global flag
  -h, --help     Print help information

❯ cargo run -q -- hoge --help       
help for hoge

Usage: subcommand hoge [OPTIONS] --opt <OPT>

Options:
  -o, --opt <OPT>  
  -v, --verbose    Global flag
  -h, --help       Print help information

❯ cargo run -q -- fuga --help
help for fuga

Usage: subcommand fuga [OPTIONS] --opt <OPT> <COMMAND>

Commands:
  nested  help for nested
  help    Print this message or the help of the given subcommand(s)

Options:
  -o, --opt <OPT>  
  -v, --verbose    Global flag
  -g, --global     global option for fuga
  -h, --help       Print help information

❯ cargo run -q -- fuga nested --help
help for nested

Usage: subcommand fuga --opt <OPT> nested [OPTIONS] --opt <OPT>

Options:
  -o, --opt <OPT>  opt for nested
  -v, --verbose    Global flag
  -g, --global     global option for fuga
  -h, --help       Print help information
```

## Success

```bash
❯ cargo run -q -- hoge --opt 1          
verbose: false
hoge 1

❯ cargo run -q -- hoge --opt 1 --verbose
verbose: true
hoge 1

❯ cargo run -q -- fuga --opt 1
verbose: false
fuga 1
global false
sub command is not specified

❯ cargo run -q -- fuga --opt 1 nested --opt 1
verbose: false
fuga 1
global false
nested 1

❯ cargo run -q -- fuga --opt 1 nested --opt 1 --global
verbose: false
fuga 1
global true
nested 1
```

## Fail

```
❯ cargo run -q -- piyo --opt 1
error: The subcommand 'piyo' wasn't recognized

Usage: subcommand <COMMAND>

For more information try '--help'

❯ cargo run -q -- hoge --global
error: Found argument '--global' which wasn't expected, or isn't valid in this context

  If you tried to supply '--global' as a value rather than a flag, use '-- --global'

Usage: subcommand hoge [OPTIONS] --opt <OPT>

For more information try '--help'

❯ cargo run -q -- fuga nested --opt 1     
error: The following required arguments were not provided:
  --opt <OPT>

Usage: subcommand fuga --opt <OPT>

For more information try '--help'
```
