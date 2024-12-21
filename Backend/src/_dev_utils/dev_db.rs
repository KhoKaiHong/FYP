use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use tracing::info;

type Db = Pool<Postgres>;

// Since this is dev environment, it is ok to hardcode the connection string.
const PG_DEV_POSTGRES_URL: &str = "postgresql://postgres:postgres@localhost:5432/postgres";
const PG_DEV_APP_URL: &str = "postgresql://dev:dev_only_pwd@localhost:5432/dev_db";

// SQL files locations
const SQL_RECREATE_DB: &str = "sql/dev_initial/00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

// Initialize the database for development.
pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("{:<12} - init_dev_db()", "FOR-DEV-ONLY");

    // Creates a new db connection pool, drops the old one, then recreates a new database.
    {
        let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
        pexec(&root_db, SQL_RECREATE_DB).await?;
    }

    // Get SQL files to be executed.
    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();
    paths.sort();

    // Execute the SQL files.
    let app_db = new_db_pool(PG_DEV_APP_URL).await?;

    for path in paths {
        if let Some(path) = path.to_str() {
            // For Windows paths
            let path = path.replace('\\', "/"); 

            // Execute SQL files that are not used for db recreation.
            if path.ends_with(".sql") && path != SQL_RECREATE_DB {
                pexec(&app_db, &path).await?;
            }
        }
    }

    Ok(())
}

// Function that executes a SQL file.
async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    info!("{:<12} - pexec: {file}", "FOR-DEV-ONLY");

    let content = fs::read_to_string(file)?;

    let sqls: Vec<&str> = content.split(';').collect();

    for sql in sqls {
        sqlx::query(sql).execute(db).await?;
    }

    Ok(())
}

// Function that creates a new database conenction pool.
async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(500))
        .connect(db_con_url)
        .await
}
