use axum::{
    Router,
    extract::{Extension, Json},
    response::Html,
    routing::{get, post},
};
use juniper::{Context, EmptyMutation, EmptySubscription, RootNode};
use std::net::SocketAddr;
use std::sync::Arc;

struct Query;

#[juniper::graphql_object(Context = MyContext)]
impl Query {
    fn hello() -> String {
        "Hello from GraphQL!".to_string()
    }
}

#[derive(Clone)]
struct MyContext;
impl Context for MyContext {}

type Schema = RootNode<'static, Query, EmptyMutation<MyContext>, EmptySubscription<MyContext>>;

pub async fn start_graphql_server() {
    println!("Starting GraphQL server on port 2401");

    let schema = Arc::new(Schema::new(
        Query,
        EmptyMutation::new(),
        EmptySubscription::new(),
    ));

    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/graphiql", get(graphiql))
        .layer(Extension(schema))
        .layer(Extension(MyContext));

    let addr = SocketAddr::from(([0, 0, 0, 0], 2401));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn graphiql() -> Html<String> {
    Html(juniper::http::graphiql::graphiql_source("/graphql", None))
}

async fn graphql_handler(
    Extension(schema): Extension<Arc<Schema>>,
    Extension(ctx): Extension<MyContext>,
    Json(request): Json<juniper::http::GraphQLRequest>,
) -> Json<juniper::http::GraphQLResponse> {
    let res = request.execute(&schema, &ctx).await;
    Json(res)
}
