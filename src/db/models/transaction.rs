use serde::{Serialize, Deserialize};
use crate::db::get_db;
use surrealdb::Error;

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    id: String,
    uuid: String,
    amount: f64,
    description: String,
    date: String,
}

