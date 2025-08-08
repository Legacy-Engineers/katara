mod api;
mod db;
use api::{graphql::start_graphql_server, grpc::start_grpc_server, rest_api::start_rest_api_server};
use db::connect_to_db;

#[tokio::main]
async fn main() {
    let db = connect_to_db().await;
    start_graphql_server();
    start_grpc_server();
    start_rest_api_server();
}
