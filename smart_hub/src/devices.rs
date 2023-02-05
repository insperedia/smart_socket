use std::fmt::format;

pub trait Device {
    fn process_command(&mut self, command: &str) -> String;
}

pub struct SmartThermometer {
    temp: i32
}

impl SmartThermometer {
    pub fn new() -> SmartThermometer {
        SmartThermometer {
            temp: 0
        }
    }
}

impl Device for SmartThermometer {
    fn process_command(&mut self, command: &str) -> String {
        if command.contains("settemp") {
            let data = command.split_once("#").expect("temp vcalue");
            let temp = data.1.parse::<i32>().unwrap();
            self.temp = temp;
            return format!("New temp: {}", temp);
        }
        "Command not found".to_string()
    }
}

pub struct SmartSocket {
    is_on: bool,
}


impl SmartSocket {
    pub fn new() -> SmartSocket {
        SmartSocket { is_on: false }
    }
}

impl Device for SmartSocket {
    fn process_command(&mut self, command: &str) -> String{
        if command == "seton" {
            self.is_on = true;
            return "Socket now is on".to_string();
        } else if command == "setoff" {
            self.is_on = false;
            return "Socket now is off".to_string();
        } else if command == "info" {
            return if self.is_on {
                "Socket is on".to_string()
            } else {
                "Socket is off".to_string()
            };
        }
        "Command not found".to_string()
    }
}

impl Default for SmartSocket {
    fn default() -> Self {
        Self::new()
    }
}
