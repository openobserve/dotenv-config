use dotenv_config::EnvConfig;
use dotenvy::dotenv;

#[derive(Debug, EnvConfig)]
struct Config {
    #[env_config(default = "192.168.2.1")]
    server_addr: String,
    server_mode: bool,
    #[env_config(name = "ZINC_FOO", default = true, help = "foo is important")]
    foo: bool,
    #[env_config(name = "ZINC_BAR", default = 123456)]
    bar: Option<i64>,
    rr: Redis,
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
    assert!(cfg.server_mode == false);
    assert!(cfg.foo == false);
    assert!(cfg.bar == Some(88888));
    assert!(cfg.rr.addr == "");
    assert!(cfg.rr.port == "");
    assert!(cfg.rr.auth == "");
    assert!(cfg.rr.timeout == 30i32);

    let help_keys = Config::get_help();
    println!("help_keys: {:?}", help_keys);
    assert!(help_keys.contains_key("ZINC_FOO"));
    assert!(help_keys.contains_key("ZINC_BAR"));

    let keys = help_keys.get("ZINC_BAR").unwrap();
    assert_eq!(keys.0, "123456"); // default value
    assert_eq!(keys.1, None); // help value

    let keys = help_keys.get("ZINC_FOO").unwrap();
    assert_eq!(keys.0, "true"); // default value
    assert_eq!(keys.1, Some("foo is important".to_string())); // help value
}
