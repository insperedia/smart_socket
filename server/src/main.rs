use std::thread;
use smart_hub::devices::{ SmartThermometer};
use smart_hub::server::Server;
use smart_hub::transport::{ UdpTransport};

fn main() {
    let handle = thread::spawn( move || {
        let transport = UdpTransport::new("localhost:2233".to_string(), "localhost:2234".to_string());
        let mut server = Server::new(transport);
        server
            .devices
           // .insert("socket1".to_string(), Box::new(SmartSocket::new()));
            .insert("therm1".to_string(), Box::new(SmartThermometer::new()));
        server.start();
    });
    handle.join().unwrap();
}
