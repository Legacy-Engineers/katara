pub mod models;

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

// trait DBModel {
//     async fn create(&self) -> Result<(), Error>;
//     async fn get(&self, id: String) -> Result<(), Error>;
//     async fn update(&self, id: String) -> Result<(), Error>;
//     async fn delete(&self, id: String) -> Result<(), Error>;
// }

pub async fn connect_to_db() -> Result<Surreal<Client>, Error> {
    println!("Attempting to connect to SurrealDB at ws://localhost:8000/rpc...");

    let db = Surreal::new::<Ws>("localhost:8000").await.map_err(|e| {
        eprintln!("Failed to connect to SurrealDB: {}", e);
        e
    })?;

    println!("Connected to SurrealDB, attempting authentication...");

    // Signin as a namespace, database, or root user
    // db.signin(Root {
    //     username: "root",
    //     password: "root",
    // }).await.map_err(|e| {
    //     eprintln!("Authentication failed: {}", e);
    //     e
    // })?;

    // println!("Authentication successful, selecting namespace and database...");

    db.use_ns("katara").use_db("katara").await.map_err(|e| {
        eprintln!("Failed to select namespace/database: {}", e);
        e
    })?;

    println!(
        "✅ Database connection successful! Connected to 'katara' namespace and database at ws://localhost:8000"
    );
    Ok(db)
}

pub async fn get_db() -> Result<Surreal<Client>, Error> {
    connect_to_db().await
}
