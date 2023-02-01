use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::{Read, Write};
use std::net::{IpAddr, TcpListener, TcpStream};
use crate::errors::TransportError;

pub struct TcpTransport {
     tcp: TcpListener,
     addr: String,
     connections: HashMap<u32, TcpStream>
}

impl TcpTransport {
    pub fn new(addr: String) -> TcpTransport {
        let tcp = TcpListener::bind(addr.as_str()).unwrap();
        TcpTransport {
            tcp,
            addr,
            connections: Default::default(),
        }
    }
}


pub trait Transport {
   // fn listen(&self, callback: A) where A: Callback ;
   fn client_command(&self, data: &str) -> Result<String, TransportError>;
    fn get_next_data(&mut self) -> (u32, String);
    fn response(&mut self, connection_id: u32, data: &str);
}

impl Transport for TcpTransport {
    fn client_command(&self, data: &str) -> Result<String, TransportError>{
        let mut stream = TcpStream::connect(&self.addr)?;
        stream.write_all(data.as_bytes())?;
        stream.flush().unwrap();
        let mut response = String::new();
        let result = stream.read_to_string(&mut response);
        match result {
            Ok(_) => {return Ok(response)}
            Err(_) => {panic!("Error reading response")}
        }
    }

    fn get_next_data(&mut self) -> (u32, String) {
        let mut data = vec![];
        let result = match self.tcp.accept() {
            Ok((mut stream, addr)) => {
                stream.read_to_end(&mut data).expect("Can not read data");
                println!("Data read: {:?}", data);
                let mut index = 0;
                loop {
                    if ! self.connections.contains_key(&index) {
                        break;
                    }
                    index = index + 1;
                    if index == u32::MAX {
                        panic!("Connection limit reached");
                    }
                }
                self.connections.insert(index, stream);
                let s = match String::from_utf8(data) {
                    Ok(data) =>  return  (index, data),
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };

            },
            Err(e) => panic!("couldn't get client: {e:?}"),
        };

    }

    fn response(&mut self, connection_id: u32, data: &str) {
        let result = self.connections.get(&connection_id);
        match result {
            None => { panic!("Connection not found")}
            Some(stream) => {
                let mut stream = stream;
                stream.write_all(data.as_bytes());
                self.connections.remove(&connection_id);
            }
        }
    }
}

