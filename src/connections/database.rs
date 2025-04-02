use migration::MigratorTrait;
use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;

use crate::config;

static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_database_connection() -> DatabaseConnection {
    DB.get_or_init(|| async {
        let db = Database::connect(config::database::get_database_config().clone())
            .await
            .expect("Failed to connect on database");

        migration::Migrator::up(&db, None)
            .await
            .expect("Failed to run migration on database");

        db
    })
    .await
    .to_owned()
}
