use smart_hub::devices::SmartThermometer;
use smart_hub::server::Server;
use smart_hub::transport::UdpTransport;
use std::thread;

#[tokio::main]
async fn main() {
        let transport =
            UdpTransport::new("localhost:2233".to_string(), "localhost:2234".to_string()).await;
        let mut server = Server::new(transport);
        server
            .devices
            // .insert("socket1".to_string(), Box::new(SmartSocket::new()));
            .insert("therm1".to_string(), Box::<SmartThermometer>::default());
        server.start().await;

}
