# positional_arg

```bash
❯ cargo run -q -- --help
Usage: positional-arg <FILE> [-- <LAST_ARGS>...]

Arguments:
  <FILE>          位置引数の説明
  [LAST_ARGS]...  ここはパースされない

Options:
  -h, --help  Print help information
```

## Success

```bash
❯ cargo run -q -- file
file
last_args: []

❯ cargo run -q -- file -- last --opt
file
last_args: ["last", "--opt"]
```

## Fail

```bash
❯ cargo run -q                      
error: The following required arguments were not provided:
  <FILE>

Usage: positional-arg <FILE> [-- <LAST_ARGS>...]

For more information try '--help'

❯ cargo run -q -- file last --opt   
error: Found argument 'last' which wasn't expected, or isn't valid in this context

Usage: positional-arg <FILE> [-- <LAST_ARGS>...]

For more information try '--help'
```