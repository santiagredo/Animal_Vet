use std::net::TcpListener;

use backend::routes::routes_config;
// use migration::{Migrator as RegularMigrator, MigratorTrait as RegularMigratorTrait};
// use migration_settings::Migrator as SettingsMigrator;
use utils::{run, Environments};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let config = get_config().await;

    // SettingsMigrator::up(&config.settings_db_url, None)
    //     .await
    //     .unwrap();

    // RegularMigrator::up(&config.db_url, None).await.unwrap();

    let listener = TcpListener::bind("127.0.0.1:8082").expect("Failed to bind local address");

    let app_port = listener.local_addr().unwrap().port();

    println!("Program started on port: {}", app_port);

    run(listener, routes_config, Environments::DEV, "Backend")?.await
}
