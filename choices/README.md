# choices

```bash
❯ cargo run -q -- --help
Usage: choices [OPTIONS]

Options:
      --log-level <LOG_LEVEL>  Log level [default: info] [possible values: debug, info, warn, error, critical]
      --fruits <FRUITS>        Fruits [possible values: apple, orange, banana]
  -h, --help                   Print help information
```

## Success

```bash
❯ cargo run -q -- --log-level debug
log_level: DEBUG
fruits: []

❯ cargo run -q -- --fruits apple,orange 
log_level: INFO
fruits: [Apple, Orange]
```

## Fail

```
❯ cargo run -q -- --log-level inavlid   
error: "inavlid" isn't a valid value for '--log-level <LOG_LEVEL>'
  [possible values: debug, info, warn, error, critical]

For more information try '--help'

❯ cargo run -q -- --fruits apple,invalid
error: "invalid" isn't a valid value for '--fruits <FRUITS>'
  [possible values: apple, orange, banana]

For more information try '--help'
```

