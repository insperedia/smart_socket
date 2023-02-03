use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
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

pub trait NetworkedStream {
    fn read_to_eol(&mut self, data: &mut String);
}

impl NetworkedStream for TcpStream {
    fn read_to_eol(&mut self, data: &mut String ) {
        let mut buffer = [0; 4];
        loop {

            let result = self.read(&mut buffer).unwrap();
            if result == 0 {
                break;
            }
            let chars = buffer.get(0..result).unwrap();
            let string = String::from_utf8_lossy(chars).to_string();
            if string.contains('\n')
            {
                data.push_str(string.split_once('\n').unwrap().0);
                break;
            } else {
                data.push_str(string.as_str());
            }
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
        stream.read_to_eol(&mut response);
        Ok(response)
    }

    fn get_next_data(&mut self) -> (u32, String) {
        let mut data = String::new();
        match self.tcp.accept() {
            Ok((mut stream, _)) => {
                stream.read_to_eol(&mut data);
                println!("Data read: {:?}", data);
                let mut index = 0;
                loop {
                    if !self.connections.contains_key(&index) {
                        break;
                    }
                    index += 1;
                    if index == u32::MAX {
                        panic!("Connection limit reached");
                    }
                }
                self.connections.insert(index, stream);
                (index, data)
            },
            Err(e) => panic!("couldn't get client: {e:?}"),
        }

    }

    fn response(&mut self, connection_id: u32, data: &str) {
        let result = self.connections.get(&connection_id);
        match result {
            None => { panic!("Connection not found")}
            Some(stream) => {
                let mut stream = stream;
                stream.write_all(data.as_bytes()).unwrap();
                stream.write_all("\n".as_bytes()).unwrap();
                self.connections.remove(&connection_id);
            }
        }
    }
}

