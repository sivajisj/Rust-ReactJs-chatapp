use std::env;

pub struct AppConfig {
    pub port: u16
}

impl AppConfig {
    pub fn init()-> Self{
        let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    AppConfig {port}
    }
}