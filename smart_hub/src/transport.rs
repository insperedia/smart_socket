use crate::errors::TransportError;
use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};

pub struct TcpTransport {
    tcp: TcpListener,
    addr: String,
    connections: HashMap<u32, TcpStream>,
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


pub struct UdpTransport {
    addr: String,
    socket: UdpSocket
}

impl UdpTransport {
    pub fn new(addr: String, remote_addr: String) -> UdpTransport {
        let socket =  UdpSocket::bind(&addr).unwrap();
        socket.connect(remote_addr).expect("connect function failed");
       UdpTransport {
           socket,
           addr
       }
    }
}

pub trait NetworkedStream {
    fn read_to_eol(&mut self, data: &mut String);
    fn read_by_size(&mut self) -> Vec<u8>;
    fn write_with_size(&mut self, data: &[u8]) -> io::Result<()>;
}


 impl NetworkedStream for TcpStream {
    fn read_to_eol(&mut self, data: &mut String) {
        //   println!("read_to_eol");
        let mut buffer = [0; 4];
        loop {
            let result = self.read(&mut buffer).unwrap();
            if result == 0 {
                break;
            }
            //     println!("read: {:?}", buffer);
            let chars = buffer.get(0..result).unwrap();
            let string = String::from_utf8_lossy(chars).to_string();
            if string.contains(3 as char) {
                data.push_str(string.split_once(3 as char).unwrap().0);
                break;
            } else {
                data.push_str(string.as_str());
            }
        }
        //   println!("data: {:?}", data);
    }

    fn read_by_size(&mut self) -> Vec<u8> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).unwrap();
        let len = u32::from_be_bytes(buf);
        let mut buf = vec![0; len as _];
        self.read_exact(&mut buf).unwrap();
        return buf;
    }

    fn write_with_size(&mut self, data: &[u8]) -> io::Result<()> {
        let bytes = data;
        let len = bytes.len() as u32;
        let len_bytes = len.to_be_bytes();
        self.write_all(&len_bytes)?;
        self.write_all(bytes)?;
        Ok(())
    }
}

pub trait Transport {
    // fn listen(&self, callback: A) where A: Callback ;
    fn client_command(&self, data: &str) -> Result<String, TransportError>;
    fn get_next_data(&mut self) -> (u32, String);
    fn response(&mut self, connection_id: u32, data: &str);
}

impl Transport for TcpTransport {
    fn client_command(&self, data: &str) -> Result<String, TransportError> {
        let mut stream = TcpStream::connect(&self.addr)?;
        stream.write_with_size(data.as_bytes())?;
    //    stream.write_all(data.as_bytes())?;
   //     stream.write_all(&[3])?;
    //    stream.flush().unwrap();
        let mut response: Vec<u8> = Vec::new() ;
        let response = stream.read_by_size();
        let response = String::from_utf8(response).unwrap();
        Ok(response)
    }

    fn get_next_data(&mut self) -> (u32, String) {
        let mut data = String::new();
        match self.tcp.accept() {
            Ok((mut stream, _)) => {
                println!("Connection accepted");
                let data = stream.read_by_size();
                let data = String::from_utf8(data).unwrap();

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
            }
            Err(e) => panic!("couldn't get client: {e:?}"),
        }
    }

    fn response(&mut self, connection_id: u32, data: &str) {
        let result = self.connections.get(&connection_id);
        match result {
            None => {
                panic!("Connection not found")
            }
            Some(stream) => {
                let mut stream = stream;
       //        stream.write_with_size(data.as_bytes()).unwrap();
                let bytes = data.as_bytes();
                let len = bytes.len() as u32;
                let len_bytes = len.to_be_bytes();
                stream.write_all(&len_bytes).unwrap();
                stream.write_all(bytes).unwrap();

     //           self.connections.remove(&connection_id);


            }
        }
    }
}

impl Transport for UdpTransport {
    fn client_command(&self, data: &str) -> Result<String, TransportError> {

        self.socket.send(data.as_bytes()).unwrap();
        println!("sent");
        let mut buf = [0; 100];
        let (size, src) = self.socket.recv_from(&mut buf).unwrap();
        let data = buf.get(0..size).unwrap();
        let string = String::from_utf8(Vec::from(data)).unwrap();
        Ok(string)
    }

    fn get_next_data(&mut self) -> (u32, String) {
        let socket = &self.socket;
        let mut buf = [0; 100];
        let (size, src) = socket.recv_from(&mut buf).unwrap();
        println!("recieved");
        let data = buf.get(0..size).unwrap();
        let string = String::from_utf8(Vec::from(data)).unwrap();
        (0, string)
    }

    fn response(&mut self, connection_id: u32, data: &str) {
        self.socket.send(data.as_bytes()).unwrap();
    }
}