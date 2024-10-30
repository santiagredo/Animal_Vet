use std::{env, str::FromStr, sync::Arc};

use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};

use tokio::sync::OnceCell;

pub struct Config {
    pub settings_db_url: DatabaseConnection,
    pub db_url: DatabaseConnection,
}

impl Config {
    async fn new() -> Self {
        dotenvy::dotenv().unwrap();

        let env_sett_db_url: String = Self::get_env_val("SETTINGS_DATABASE_URL");
        let sett_db = Database::connect(&env_sett_db_url).await.unwrap();

        let env_db_url: String = Self::get_env_val("DATABASE_URL");
        let db = Database::connect(&env_db_url).await.unwrap();

        Config {
            settings_db_url: sett_db,
            db_url: db,
        }
    }

    fn get_env_val<T: FromStr>(key: &str) -> T
    where
        T::Err: std::fmt::Debug,
    {
        env::var(key)
            .expect(&format!("{key} not set"))
            .parse::<T>()
            .expect(&format!("Can't parse {key} to desired type"))
    }

    async fn new_test() -> Self {
        dotenvy::dotenv().unwrap();

        let env_sett_db_url: String = Self::get_env_val("SETTINGS_DATABASE_URL");
        let sett_db = Database::connect(&env_sett_db_url).await.unwrap();

        let env_db_url: String = Self::get_env_val("DATABASE_URL");
        let db = Database::connect(&env_db_url).await.unwrap();

        Config {
            settings_db_url: sett_db,
            db_url: db,
        }
    }
}

static CONFIG: OnceCell<Arc<bool>> = OnceCell::const_new();

pub async fn get_config() -> Config {
    if dotenv().is_err() {
        panic!("Error loading environment variables")
    }

    match CONFIG.get() {
        None => Config::new().await,
        Some(val) => {
            if !**val {
                Config::new().await
            } else {
                Config::new_test().await
            }
        }
    }
}

pub async fn get_test_config() -> Config {
    if dotenv().is_err() {
        panic!("Error loading environment variables")
    }

    CONFIG.get_or_init(|| async { Arc::new(true) }).await;

    Config::new_test().await
}
