use std::borrow::Borrow;
use std::cell::Cell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::net::TcpStream;
use crate::devices::Device;
use crate::transport::Transport;

pub struct Server<A: Transport> {
    transport: A,
    pub devices: HashMap<String, Box<dyn Device>>,
}

impl<A: Transport> Server<A> {
    pub fn new(transport: A) -> Server<A> {
        Server {
            transport,
            devices: HashMap::new(),
        }
    }

    pub fn start(&mut self) {
        let mut local_self = Cell::new(self);
        loop {
            let server = local_self.get_mut();
            let data = server.transport.get_next_data();
            let result = server.process_command(&data.1);
            server.transport.response(data.0, result.as_str());
        }
    }

    fn process_command(&mut self, data: &str) -> String {
        return if data == "info" {
            let mut result: Vec<String> = vec!["Devices: \n".to_string()];
            for (key, _) in &self.devices {
                let line = format!("{}\n", key);
                result.push(line);
            }
            result.join("\n")
        } else {
            let command_option = data.split_once("|||");
            match command_option {
                None => { "Incorrect data format".to_string() }
                Some(command) => {
                    let device = self.devices.get_mut(command.0);
                    match device {
                        None => {
                            "Device not found".to_string()
                        }
                        Some(mut device) => {
                            let result = device.process_command(command.1);
                            println!("{}", result);
                            result.to_string()
                        }
                    }
                }
            }
        };
    }


}