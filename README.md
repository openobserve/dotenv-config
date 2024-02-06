# Dot Env Config

use `.env` as config file and parse environments to config struct.

## Usage

### derive EnvConfig

```rust
use dotenv_config::EnvConfig;
use dotenvy::dotenv;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl std::str::FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err("no match color"),
        }
    }
}

#[derive(Debug, EnvConfig)]
struct Config {
    #[env_config(default = "192.168.2.1")]
    server_addr: String,
    server_mode: bool,
    #[env_config(name = "ZINC_ENABLE", default = true)]
    enable: bool,
    #[env_config(name = "ZINC_NUMBER", default = 123456, help = "this is for demo")]
    num: Option<i64>,
    #[env_config(parse, default="green")] // or parse=true
    color: Color,
}

fn main() {
    dotenv().ok();
    let cfg = Config::init().unwrap();
    println!("{:#?}", cfg);

    // print config help
    let help = Config::get_help();
    println!("{:#?}", help);
}
```

### attribute env_config

you can use macro attribute set field attribute 

- name: change default environment key
- default: if not set, used as default value

## you can though system environments or `.env` file config it.

```
ZINC_ENABLE=false
ZINC_NUMBER=8787878
```

default load environment key is: `structName_fieldName` do UpperSnake, like above struct, default config key is:

```
CONFIG_SERVER_ADDR
CONFIG_SERVER_MODE
ZINC_ENABLE
ZINC_NUMBER
```
