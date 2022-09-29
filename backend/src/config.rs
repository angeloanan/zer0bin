use std::{env, fs::File, path::PathBuf};

use num_traits::Num;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub pastes: PastesConfig,
    pub ratelimits: RatelimitsConifg,
    pub databases: DatabasesConfig,
    pub logging: LoggingConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub backend_host: String,
    pub backend_port: u16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RatelimitsConifg {
    pub seconds_in_between_pastes: u64,
    pub allowed_pastes_before_ratelimit: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PastesConfig {
    pub character_limit: usize,
    pub days_til_expiration: i64,
    pub id_length: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DatabasesConfig {
    pub postgres_uri: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoggingConfig {
    pub on_post_paste: bool,
    pub on_get_paste: bool,
}

pub fn load(path: PathBuf) -> Config {
    let file = File::open(path);
    match file {
        Ok(f) => serde_json::from_reader(f).unwrap(),
        Err(_) => Config {
            server: ServerConfig {
                backend_host: "0.0.0.0".to_owned(),
                backend_port: parse_env_int("PORT", 8000),
            },
            pastes: PastesConfig {
                character_limit: parse_env_int("CHARLIMIT", 50000),
                days_til_expiration: parse_env_int("EXPIRATION_DAYS", -1),
                id_length: parse_env_int("ID_LENGTH", 6),
            },
            ratelimits: RatelimitsConifg {
                seconds_in_between_pastes: parse_env_int("PASTE_POST_RATE_LIMIT", 2),
                allowed_pastes_before_ratelimit: parse_env_int("PASTE_BEFORE_RATE_LIMIT", 5),
            },
            databases: DatabasesConfig {
                postgres_uri: env::var("POSTGRES_URI").expect("POSTGRES_URI is not set"),
            },
            logging: LoggingConfig {
                on_post_paste: parse_env_bool("LOG_ON_POST", false),
                on_get_paste: parse_env_bool("LOG_ON_GET", false),
            },
        },
    }
}

fn parse_env_int<T: Num>(env_name: &str, default: T) -> T
where
    <T as Num>::FromStrRadixErr: std::fmt::Debug,
{
    match std::env::var(env_name) {
        Ok(v) => T::from_str_radix(v.as_str(), 10).unwrap(),
        Err(_) => default,
    }
}

fn parse_env_bool(env_name: &str, default: bool) -> bool {
    match std::env::var(env_name) {
        Ok(v) => v == "true",
        Err(_) => default,
    }
}
