use std::process::exit;

use figment::{Figment, providers::Env};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    db_string: Option<String>,
}
fn main() {
    const BUILT_ENV: &str = match option_env!("APP_ENV") {
        Some(v) => v,
        None => "development",
    };
    let file_name = format!(".env.{}", BUILT_ENV);
    let Ok(_) = dotenvy::from_filename(&file_name) else {
        eprintln!("cannot read `{}`", file_name);
        exit(1);
    };

    let config: Config = Figment::new()
        .merge(Env::prefixed("AWA_"))
        .extract()
        .expect("Cannot read the config");

    let Some(db_string) = config.db_string else {
        eprintln!("the DB_STRING cannot be null");
        exit(1);
    };
    print!("{}", db_string);
}
