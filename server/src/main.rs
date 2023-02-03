use smart_hub::devices::SmartSocket;
use smart_hub::server::Server;
use smart_hub::transport::TcpTransport;

fn main() {
    let transport = TcpTransport::new("localhost:2233".to_string());
    let mut server = Server::new(transport);

    server
        .devices
        .insert("socket1".to_string(), Box::new(SmartSocket::new()));
    server.start();
}
