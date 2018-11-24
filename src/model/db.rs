use actix_web::actix::{Actor, Addr, SyncArbiter, SyncContext};
use diesel::prelude::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv;
use slog::Logger;

pub struct Database {
    pub log: Logger,
    pub connection: Pool<ConnectionManager<PgConnection>>,
}

impl Actor for Database {
    type Context = SyncContext<Self>;
}

embed_migrations!();

pub fn init(logger: Logger) -> Addr<Database> {
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let conn = &pool.get().unwrap();
    embedded_migrations::run(&*conn);
    SyncArbiter::start(4, move || Database {
        log: logger.clone(),
        connection: pool.clone(),
    })
}
