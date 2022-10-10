# flag

```bash
❯ cargo run -q -- --help                         
Usage: flag [OPTIONS]

Options:
  -v, --verbose      
  -n, --non-verbose  
  -h, --help         Print help information
```

## Success

```bash
❯ cargo run -q          
verbose: false
non-verbose: true

❯ cargo run -q -- -v    
verbose: true
non-verbose: true

❯ cargo run -q -- -v -n
verbose: true
non-verbose: false
```

## Fail

```bash
❯ cargo run -q -- -v hoge
error: Found argument 'hoge' which wasn't expected, or isn't valid in this context

Usage: flag [OPTIONS]

For more information try '--help'
```
