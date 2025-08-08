use serde::{Serialize, Deserialize};
use crate::db::get_db;
use crate::db::DBModel;
use surrealdb::Error;

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    id: String,
    uuid: String,
    amount: f64,
    description: String,
    date: String,
}

impl DBModel for Transaction {
    async fn create(&self) -> Result<(), Error> {
        let db = get_db().await;
        let created = db.create(("transaction", self.uuid)).content(self).await.unwrap();
        return Ok(created);
    }
}