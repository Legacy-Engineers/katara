use tonic::async_trait;
use tonic::codegen::BoxFuture;
use tonic::{Request, Response, Status, transport::Server};

// Define request and response structs manually
#[derive(Debug, Default)]
pub struct HelloRequest {
    pub name: String,
}

#[derive(Debug, Default)]
pub struct HelloReply {
    pub message: String,
}

// Define a gRPC service trait manually
#[async_trait]
pub trait Greeter: Send + Sync + 'static {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status>;
}

// Implement the service
#[derive(Default)]
pub struct MyGreeter;

#[async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        Ok(Response::new(HelloReply {
            message: format!("Hello, {}!", request.into_inner().name),
        }))
    }
}

// Minimal tonic server without proto generation
pub async fn start_grpc_server() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting gRPC server on 0.0.0.0:50051");

    let addr = "0.0.0.0:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        // We'll replace this with an actual service once proto is ready
        .add_service(tonic_reflection::server::Builder::configure().build_v1()?)
        .serve(addr)
        .await?;

    Ok(())
}
