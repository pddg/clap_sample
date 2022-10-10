# optional_arg

```bash
❯ cargo run -q -- --help 
Usage: optional-arg [OPTIONS] --name <NAME> --count <COUNT>

Options:
  -n, --name <NAME>      your name
  -c, --count <COUNT>    a 32bit integer
  -o, --opt <OPT>        optional value
  -i, --inputs <INPUTS>  multiple inputs
  -h, --help             Print help information
```

## Success

```bash
❯ cargo run -q -- -n hoge -c 1
name: hoge
count: 1
opt: not specified
inputs: []

❯ cargo run -q -- -n hoge -c 1 -o fuga -i 1 -i 2
name: hoge
count: 1
opt: fuga
inputs: ["1", "2"]
```

## Fail

```bash
❯ cargo run -q                                  
error: The following required arguments were not provided:
  --name <NAME>
  --count <COUNT>

Usage: optional-arg --name <NAME> --count <COUNT>

For more information try '--help'

❯ cargo run -q -- -n                            
error: The argument '--name <NAME>' requires a value but none was supplied

For more information try '--help'
```
