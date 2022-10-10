# env_var

```bash
❯ cargo run -q -- --help                
Usage: env-var --from-env <FROM_ENV> --credential <CREDENTIAL> --hidden-env <HIDDEN_ENV>

Options:
  -f, --from-env <FROM_ENV>      Value from env [env: FROM_ENV=]
  -c, --credential <CREDENTIAL>  Any credential [env: CREDENTIAL]
  -n, --hidden-env <HIDDEN_ENV>  This can be specified by env var
  -h, --help                     Print help information
```

## Success

```bash
❯ FROM_ENV=hoge CREDENTIAL=password HIDDEN_ENV=hidden cargo run -q
from_env: hoge
credential: password
hidden: hidden

❯ FROM_ENV=hoge CREDENTIAL=password HIDDEN_ENV=hidden cargo run -q -- --help
Usage: env-var --from-env <FROM_ENV> --credential <CREDENTIAL> --hidden-env <HIDDEN_ENV>

Options:
  -f, --from-env <FROM_ENV>      Value from env [env: FROM_ENV=hoge]
  -c, --credential <CREDENTIAL>  Any credential [env: CREDENTIAL]
  -n, --hidden-env <HIDDEN_ENV>  This can be specified by env var
  -h, --help                     Print help information

❯ FROM_ENV=hoge CREDENTIAL=password HIDDEN_ENV=hidden cargo run -q -- --from-env overwrite
from_env: overwrite
credential: password
hidden: hidden
```

## Fail

```bash
❯ cargo run -q                                                                            
error: The following required arguments were not provided:
  --from-env <FROM_ENV>
  --credential <CREDENTIAL>
  --hidden-env <HIDDEN_ENV>

Usage: env-var --from-env <FROM_ENV> --credential <CREDENTIAL> --hidden-env <HIDDEN_ENV>

For more information try '--help'
```
