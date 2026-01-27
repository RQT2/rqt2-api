pub mod rqt2_api {
    tonic::include_proto!("rqt2.api.v1"); 
}

//use rqt2_api::build_service_server::BuildService; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Iniciando el core de RQT2...");
    Ok(())
}
