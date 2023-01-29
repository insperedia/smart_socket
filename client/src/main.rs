use smart_hub::client::Client;
use smart_hub::transport::TcpTransport;

fn main() {
    let transport = TcpTransport::new("localhost:2233".to_string());

    let client = Client {
        transport,
    };

    client.send("socket1|||seton");

}
