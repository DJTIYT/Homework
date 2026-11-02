mod entities;
use entities::*;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, EntityTrait, Schema};

use std::path::Path;

use tokio;

// Adds info!(), warn!() and error!()
mod logger;

async fn add_table<E: EntityTrait>(db: &DatabaseConnection, entity: E) -> Result<(), DbErr> {
    let backend = db.get_database_backend();

    let mut stmt = Schema::new(backend).create_table_from_entity(entity);
    stmt.if_not_exists();
    db.execute(backend.build(&stmt)).await?;

    Ok(())
}

async fn init_database() -> Result<DatabaseConnection, DbErr> {
    let db_path = "homework.db";
    let db_url = format!("sqlite://{}?mode=rwc", db_path);

    if !Path::new(db_path).exists() {
        warn!(
            "Server",
            "Database '{}' does not exist, initializing new database", db_path
        );
    }

    let db = Database::connect(&db_url).await?;

    add_table(&db, class::Entity).await?;
    add_table(&db, user::Entity).await?;
    add_table(&db, subject::Entity).await?;
    add_table(&db, homework::Entity).await?;
    add_table(&db, user_homework_status::Entity).await?;

    Ok(db)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Server", "Init database");

    let db = match init_database().await {
        Ok(connection) => {
            info!("Server", "Database initialized successfully.");
            connection
        }
        Err(err) => {
            error!("Server", "Database init error: {}", err);
            std::process::exit(1);
        }
    };

    Ok(())
}
