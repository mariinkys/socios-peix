#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};

#[cfg(feature = "ssr")]
pub async fn init_database(database_url: &str) -> Result<Pool<Sqlite>, anyhow::Error> {
    if let Some(filename) = &database_url.strip_prefix("sqlite:") {
        if !std::path::Path::new(filename).exists() {
            // Create the file
            std::fs::File::create(filename)?;
            println!("Database '{filename}' created.");
        } else {
            println!("Database '{filename}' already exists.");
        }
    };
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;

    match sqlx::migrate!("./migrations").run(&pool).await {
        std::result::Result::Ok(_) => println!("Migrations run successfully"),
        Err(err) => {
            eprintln!("Error occurred running migrations: {err}");
            std::process::exit(1);
        }
    };

    Ok(pool)
}
