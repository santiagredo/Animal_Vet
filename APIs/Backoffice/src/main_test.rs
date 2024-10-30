use std::{net::TcpListener, sync::Arc};

use crate::routes::routes_config;
use ::utils::{get_test_config, run};
use migration::{Migrator as RegularMigrator, MigratorTrait as RegularMigratorTrait};
use migration_settings::Migrator as SettingsMigrator;
use tokio::sync::OnceCell;
use utils::Environments;

static APP_INSTANCE: OnceCell<Arc<String>> = OnceCell::const_new();

pub async fn spawn_test_app() -> Arc<String> {
    APP_INSTANCE
        .get_or_init(|| async {
            let config = get_test_config().await;

            SettingsMigrator::up(&config.settings_db_url, None)
                .await
                .unwrap();

            RegularMigrator::up(&config.db_url, None).await.unwrap();

            let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

            let port = listener.local_addr().unwrap().port();

            let server =
                run(listener, routes_config, Environments::DEV, "Backoffice_tests").expect("Failed to bind address");

            tokio::spawn(server);

            Arc::new(format!("http://127.0.0.1:{port}"))
        })
        .await
        .clone()
}
