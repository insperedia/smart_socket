
pub trait Device {
    fn process_command(&mut self, command: &str) -> &str;
}

pub struct SmartSocket {
    is_on: bool
}

impl SmartSocket {
    pub fn new() -> SmartSocket{
        SmartSocket {
            is_on: false,
        }
    }
}

impl Device for SmartSocket {

    fn process_command(&mut self, command: &str) -> &str {
        if command == "seton" {
            self.is_on = true;
            return "Socket now is on"
        } else if command == "setoff" {
            self.is_on = false;
            return "Socket now is off"
        } else if command == "info" {
            return if self.is_on {
                "Socket is on"
            } else {
                "Socket is off"
            }
        }
        "Command not found"
    }
}

impl Default for SmartSocket {
    fn default() -> Self {
        Self::new()
    }
}

