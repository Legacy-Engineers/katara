pub mod transaction;
use surrealdb::{Surreal, Error};
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::{Client, Ws};


trait DBModel {
    async fn create(&self) -> Result<(), Error>;
    async fn get(&self, id: String) -> Result<(), Error>;
    async fn update(&self, id: String) -> Result<(), Error>;
    async fn delete(&self, id: String) -> Result<(), Error>;
}

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

pub async fn get_db() -> Surreal<Client> {
    let db = connect_to_db().await;
    return db;
}