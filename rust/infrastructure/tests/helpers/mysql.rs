use clean_architecture_infrastructure::database::mysql::Db;
use rand::distributions::{Alphanumeric, DistString};
use sqlx::{mysql::MySqlPoolOptions, Connection, MySql, MySqlConnection, MySqlPool};
use std::{sync::Arc, time::Duration};

#[derive(Debug)]
pub struct TestMySQL {
    url: String,
    db: Db,
}

/// Sets up a new DB for running tests with.
impl TestMySQL {
    pub async fn new() -> Self {
        let db_url = Self::url();

        Self::create_database(&db_url).await;
        Self::run_migrations(&db_url).await;

        let db = Db {
            pool: Arc::new(MySqlPool::connect(&db_url).await.unwrap()),
        };

        Self { url: db_url, db }
    }

    pub fn database(&self) -> Db {
        self.db.clone()
    }

    /// Drop database after the test
    pub async fn drop_database(&self) {
        let (_conn, db_name) = Self::parse_url(&self.url);

        let pool = MySqlPoolOptions::new()
            .max_connections(1)
            .min_connections(1)
            .max_lifetime(Some(Duration::from_secs(5)))
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(5))
            .test_before_acquire(false)
            .connect(&self.url)
            .await
            .expect("error during MySQL pool creation");

        let sql = format!(
            r#"
            SELECT
            CONCAT('KILL ', id, ';')
            FROM INFORMATION_SCHEMA.PROCESSLIST
            WHERE `db` = '{}'"#,
            &db_name
        );
        sqlx::query::<MySql>(&sql)
            .execute(&pool)
            .await
            .expect("error during killing database processes");

        let sql = format!(r#"DROP DATABASE `{}`"#, &db_name);
        sqlx::query::<MySql>(&sql)
            .execute(&pool)
            .await
            .expect("error when dropping database");
    }

    /// Create database
    async fn create_database(url: &str) {
        let (conn, db_name) = Self::parse_url(url);
        let mut pool = MySqlConnection::connect(conn).await.unwrap();

        let sql = format!(r#"CREATE DATABASE `{}`"#, &db_name);
        sqlx::query::<MySql>(&sql).execute(&mut pool).await.unwrap();
    }

    /// Launch migrations
    async fn run_migrations(url: &str) {
        let (conn, db_name) = Self::parse_url(url);
        let mut pool = MySqlConnection::connect(&format!("{}/{}", conn, db_name))
            .await
            .unwrap();

        // Run the migrations
        sqlx::migrate!("../infrastructure/migrations")
            .run(&mut pool)
            .await
            .expect("Failed to migrate the database");
    }

    /// Generate url with a random database name
    fn url() -> String {
        dotenvy::dotenv().ok();

        // Set up the database per tests
        let suffix: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL missing from environment.");

        format!("{}_{}", db_url, suffix)
    }

    /// Parse database URL and return the database name in a separate variable
    fn parse_url(url: &str) -> (&str, &str) {
        let separator_pos = url.rfind('/').unwrap();
        let conn = &url[..=separator_pos];
        let name = &url[separator_pos + 1..];

        (conn, name)
    }
}

impl Drop for TestMySQL {
    fn drop(&mut self) {
        // Drop the DB Pool
        std::thread::scope(|s| {
            s.spawn(|| {
                let runtime = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                runtime.block_on(self.drop_database());
            });
        });
    }
}
