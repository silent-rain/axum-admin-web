use grpc_api::helloworld::greeter_client::GreeterClient;
use grpc_api::helloworld::{HelloRequest, StatusRequest};
use tonic::transport::Channel;
use tonic::Code;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let channel = Channel::from_static("http://[::1]:50051").connect().await?;
    let channel = Channel::from_static("http://127.0.0.1:50051")
        .connect()
        .await?; //Devskim: ignore DS137138

    let channel = ServiceBuilder::new().service(channel);

    let mut client = GreeterClient::new(channel);
    {
        let request = tonic::Request::new(HelloRequest {
            name: "Tonic".into(),
        });

        let response = client.say_hello(request).await?;

        println!("RESPONSE={:?}", response);
    }
    {
        let request = tonic::Request::new(StatusRequest {
            code: Code::NotFound.into(),
            message: "not found...".into(),
        });

        let response = client.say_status(request).await;

        println!("RESPONSE={:?}", response);
    }
    {
        let request = tonic::Request::new(StatusRequest {
            code: Code::DeadlineExceeded.into(),
            message: "deadline...".into(),
        });

        let response = client.say_status(request).await;

        println!("RESPONSE={:?}", response);
    }

    Ok(())
}
