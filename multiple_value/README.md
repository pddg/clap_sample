# multiple_value

```bash
❯ cargo run -q -- --help 
Usage: multiple-value [OPTIONS]

Options:
  -m, --multi <MULTI>  comma separated values are allowed [env: MULTI=]
  -h, --help           Print help information
```

## Success

```bash
❯ cargo run -q          
[]

❯ cargo run -q -- --multi hoge,fuga
["hoge", "fuga"]

❯ cargo run -q -- --multi hoge --multi fuga
["hoge", "fuga"]

❯ cargo run -q -- --multi hoge --multi fuga --multi piyo,poyo
["hoge", "fuga", "piyo", "poyo"]

❯ MULTI=hoge,fuga cargo run -q
["hoge", "fuga"]
```

## Fail

```bash
❯ cargo run -q -- --multi                                    
error: The argument '--multi <MULTI>' requires a value but none was supplied

For more information try '--help'
```
