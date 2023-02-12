use crate::errors::TransportError;
use std::collections::HashMap;
use std::io;
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use async_trait::async_trait;

pub struct TcpTransport {
    tcp: TcpListener,
    addr: String,
    connections: HashMap<u32, TcpStream>,
}

impl TcpTransport {
    pub async fn new(addr: String) -> TcpTransport {
        let tcp = TcpListener::bind(addr.as_str()).await.unwrap();
        TcpTransport {
            tcp,
            addr,
            connections: Default::default(),
        }
    }
}

pub struct UdpTransport {
    socket: UdpSocket,
}

impl UdpTransport {
    pub async fn new(addr: String, remote_addr: String) -> UdpTransport {
        let socket = UdpSocket::bind(addr).await.unwrap();
        socket
            .connect(remote_addr).await
            .expect("connect function failed");
        UdpTransport { socket }
    }
}


#[async_trait]
pub trait NetworkedStream {
    async fn read_to_eol(&mut self, data: &mut String);
    async fn read_by_size(&mut self) -> Vec<u8>;
    async fn write_with_size(&mut self, data: &[u8]) -> io::Result<()>;
}

#[async_trait]
impl NetworkedStream for TcpStream {
    async fn read_to_eol(&mut self, data: &mut String) {
        //   println!("read_to_eol");
        let mut buffer = [0; 4];
        loop {
            self.readable().await.unwrap();
            let result = self.try_read(&mut buffer).unwrap();
           // let result = self.stream.read_buf(&mut buffer).await.unwrap();
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

    async fn read_by_size(&mut self) -> Vec<u8> {
        let mut buf = [0; 4];
        self.readable().await.unwrap();
        self.try_read(&mut buf).unwrap();
        let len = u32::from_be_bytes(buf);
        let mut buf = vec![0; len as _];
        self.try_read(&mut buf).unwrap();
        buf
    }

    async fn write_with_size(&mut self, data: &[u8]) -> io::Result<()> {
        let bytes = data;
        let len = bytes.len() as u32;
        let len_bytes = len.to_be_bytes();
        self.writable().await.unwrap();
        self.try_write(&len_bytes)?;
        self.try_write(bytes)?;
        Ok(())
    }
}

#[async_trait]
pub trait Transport {
    // fn listen(&self, callback: A) where A: Callback ;
    async fn client_command(&self, data: &str) -> Result<String, TransportError>;
    async fn get_next_data(&mut self) -> (u32, String);
    async fn response(&mut self, connection_id: u32, data: &str);
}

#[async_trait]
impl Transport for TcpTransport {
    async fn client_command(&self, data: &str) -> Result<String, TransportError> {
        let mut stream = TcpStream::connect(&self.addr).await?;
        stream.write_with_size(data.as_bytes()).await?;
        //    stream.write_all(data.as_bytes())?;
        //     stream.write_all(&[3])?;
        //    stream.flush().unwrap();
        let response = stream.read_by_size().await;
        let response = String::from_utf8(response).unwrap();
        Ok(response)
    }

    async fn get_next_data(&mut self) -> (u32, String) {
        match self.tcp.accept().await {
            Ok((mut stream, _)) => {
                println!("Connection accepted");
                let data = stream.read_by_size().await;
                let data = String::from_utf8(data).unwrap();

                println!("Data read: {data:?}");
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

    async fn response(&mut self, connection_id: u32, data: &str) {
        let result = self.connections.get_mut(&connection_id);
        match result {
            None => {
                panic!("Connection not found")
            }
            Some(stream) => {
                let stream = stream;
                //        stream.write_with_size(data.as_bytes()).unwrap();
                let bytes = data.as_bytes();
                stream.write_with_size(bytes).await.unwrap();

                //           self.connections.remove(&connection_id);
            }
        }
    }
}

#[async_trait]
impl Transport for UdpTransport {
    async fn client_command(&self, data: &str) -> Result<String, TransportError> {
        self.socket.send(data.as_bytes()).await?;
        println!("sent");
        let mut buf = [0; 100];
        let (size, _) = self.socket.recv_from(&mut buf).await?;
        let data = buf.get(0..size).unwrap();
        let string = String::from_utf8(Vec::from(data)).unwrap();
        Ok(string)
    }

    async fn get_next_data(&mut self) -> (u32, String) {
        let socket = &self.socket;
        let mut buf = [0; 100];
        let (size, _) = socket.recv_from(&mut buf).await.unwrap();
        let data = buf.get(0..size).unwrap();
        let string = String::from_utf8(Vec::from(data)).unwrap();
        (0, string)
    }

    async fn response(&mut self, _connection_id: u32, data: &str) {
        self.socket.send(data.as_bytes()).await.unwrap();
    }
}
