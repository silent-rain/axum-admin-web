use grpc_api::helloworld::greeter_server::{Greeter, GreeterServer};
use grpc_api::helloworld::{HelloReply, HelloRequest, StatusRequest};
use grpc_api::FILE_DESCRIPTOR_SET;
use tonic::Code;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }

    async fn say_status(&self, request: Request<StatusRequest>) -> Result<Response<()>, Status> {
        let request = request.into_inner();
        tracing::info!("ask to return status : {}", request.code);
        Err(Status::new(Code::from(request.code), request.message))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    let (_, health_service) = tonic_health::server::health_reporter();
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()?;

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        // create trace for every request including health_service
        .add_service(health_service)
        .add_service(reflection_service)
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
