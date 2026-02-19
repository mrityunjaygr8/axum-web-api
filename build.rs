use std::env;

fn main() {
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());

    if profile == "release" {
        println!("cargo:rustc-env=APP_ENV=production")
    } else {
        println!("cargo:rustc-env=APP_ENV=development")
    };
}
