use diesel::r2d2::{ConnectionManager, PooledConnection};

type DbCon = diesel::PgConnection;

pub type Pool = diesel::r2d2::Pool<ConnectionManager<DbCon>>;
pub type Connection = PooledConnection<ConnectionManager<DbCon>>;

const MAX_CONNECTIONS: u32 = 1;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool,
}

impl Database {
    pub fn new(database_url: String) -> Database {
        let database_pool = Pool::builder()
            .connection_timeout(std::time::Duration::from_secs(5))
            .max_size(MAX_CONNECTIONS)
            .build(ConnectionManager::new(database_url))
            .unwrap();

        Database {
            pool: database_pool,
        }
    }

    pub fn get_connection(&self) -> Connection {
        self.pool.get().unwrap()
    }
}
