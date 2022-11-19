pub mod config {
    #[derive(Debug)]
    pub struct Database {
        pub url: String,
    }

    #[derive(Debug)]
    pub struct Config {
        pub database: Database,
    }

    pub fn load_config() -> Config {
        dotenv::dotenv().expect("Failed to read .env file");

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let config = Config {
            database: Database { url: database_url },
        };
        config
    }
}
