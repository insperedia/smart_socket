use crate::errors::TransportError;
use crate::transport::Transport;

pub struct Client<A: Transport> {
    pub transport: A
}

impl<A: Transport> Client<A> {
    pub fn send(&self, data: &str) -> Result<String, TransportError> {
       self.transport.client_command(data)
    }
}