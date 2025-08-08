pub mod transaction;
use surrealdb::{Surreal};
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::{Client, Ws};


pub async fn connect_to_db() -> Surreal<Client> {
    let db = Surreal::new::<Ws>("localhost:8000").await.unwrap();

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    }).await.unwrap();

    db.use_ns("katara").use_db("katara").await.unwrap();
    return db;
}