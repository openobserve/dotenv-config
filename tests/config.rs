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
    #[env_config(name = "ZINC_ENABLE", default = true, help = "this is important")]
    enable: bool,
    #[env_config(name = "ZINC_NUMBER", default = 123456)]
    num: Option<i64>,
    rr: Redis,
    #[env_config(parse, default = "green")]
    color: Color,
}

#[derive(Debug, EnvConfig)]
struct Redis {
    addr: String,
    port: String,
    auth: String,
    #[env_config(name = "ZINC_REDIS_TIMEOUT", default = 30)]
    timeout: i32,
}

#[test]
fn test_config() {
    dotenv().ok();
    let cfg = Config::init().unwrap();
    assert!(cfg.server_addr == "192.168.2.1");
    assert!(!cfg.server_mode);
    assert!(!cfg.enable);
    assert!(cfg.num == Some(88888));
    assert!(cfg.rr.addr.is_empty());
    assert!(cfg.rr.port.is_empty());
    assert!(cfg.rr.auth.is_empty());
    assert!(cfg.rr.timeout == 30i32);
    assert!(cfg.color == Color::Green);

    let help_keys = Config::get_help();
    println!("help_keys: {:?}", help_keys);
    assert!(help_keys.contains_key("ZINC_ENABLE"));
    assert!(help_keys.contains_key("ZINC_NUMBER"));

    let keys = help_keys.get("ZINC_NUMBER").unwrap();
    assert_eq!(keys.0, "123456"); // default value
    assert_eq!(keys.1, None); // help value

    let keys = help_keys.get("ZINC_ENABLE").unwrap();
    assert_eq!(keys.0, "true"); // default value
    assert_eq!(keys.1, Some("this is important".to_string())); // help value
}
