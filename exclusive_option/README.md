# exclusive_option

```bash
❯ cargo run -q -- --help 
Usage: exclusive-option <--input <INPUT>|--input-from-stdin>

Options:
      --input <INPUT>     input from arg
      --input-from-stdin  input from stdin
  -h, --help              Print help information
```

## Success

```bash
❯ cargo run -q -- --input-from-stdin          
input: not specified, input_from_stdin: true

❯ cargo run -q -- --input hoge
input: hoge, input_from_stdin: false
```

## Fail

```bash
❯ cargo run -q -- --input hoge --input-from-stdin
error: The argument '--input <INPUT>' cannot be used with '--input-from-stdin'

Usage: exclusive-option <--input <INPUT>|--input-from-stdin>

For more information try '--help'

❯ cargo run -q                                   
error: The following required arguments were not provided:
  <--input <INPUT>|--input-from-stdin>

Usage: exclusive-option <--input <INPUT>|--input-from-stdin>

For more information try '--help'
```

