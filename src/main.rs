mod api;
mod db;
use api::{
    graphql::start_graphql_server, grpc::start_grpc_server, rest_api::start_rest_api_server,
};
use db::connect_to_db;
use tokio;

#[tokio::main]
async fn main() {
    let db = connect_to_db()
        .await
        .expect("Failed to connect to the database");

    tokio::join!(
        start_rest_api_server(db),
        start_graphql_server(),
        start_grpc_server(),
    );
}
