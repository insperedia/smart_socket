use crate::transport::Transport;

pub struct Client<A: Transport> {
    pub transport: A
}

impl<A: Transport> Client<A> {
    pub fn send(&self, data: &str) {

        self.transport.client_send(data);
    }
}