use dotenv_config::EnvConfig;
use dotenvy::dotenv;

#[derive(Debug, EnvConfig)]
struct Config {
    #[env_config(name = "ZINC_SERVER_ADDR", default = "192.168.2.1")]
    server_addr: String,
    server_mode: bool,
    #[env_config(name = "ZINC_FOO", default = true)]
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

fn main() {
    dotenv().ok();
    let cfg = Config::init().unwrap();
    println!("{:#?}", cfg);
    assert!(cfg.server_addr == "192.168.2.1");
    assert!(cfg.server_mode == false);
    assert!(cfg.foo == false);
    assert!(cfg.bar == Some(88888));
    assert!(cfg.rr.addr == "");
    assert!(cfg.rr.port == "");
    assert!(cfg.rr.auth == "");
    assert!(cfg.rr.timeout == 30i32);
}
