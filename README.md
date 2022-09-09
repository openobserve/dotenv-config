# Dot Env Config

use `.env` as config file and parse environments to config struct.

## Usage

### derive EnvConfig

```rust
use dotenv_config::EnvConfig;

#[derive(Debug, EnvConfig)]
struct Config {
    #[env_config(default = "192.168.2.1")]
    server_addr: String,
    server_mode: bool,
    #[env_config(name = "ZINC_FOO", default = true)]
    foo: bool,
    #[env_config(name = "ZINC_BAR", default = 123456)]
    bar: Option<i64>,
}

fn main() {
    let cfg = Config::init().unwrap();
    println!("{:#?}", cfg);
}
```

### attribute env_config

you can use macro attribute set field attribute 

- name: change default environment key
- default: if not set, used as default value

## you can though system environments or `.env` file config it.

```
ZINC_FOO=false
ZINC_BAR=8787878
```

default load environment key is: `structName_fieldName` do UpperSnake, like above struct, default config key is:

```
CONFIG_SERVER_ADDR
CONFIG_SERVER_MODE
ZINC_FOO
ZINC_BAR
```
