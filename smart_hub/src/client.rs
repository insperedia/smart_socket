use crate::errors::TransportError;
use crate::transport::Transport;

pub struct Client<A: Transport> {
    pub transport: A,
}

impl<A: Transport> Client<A> {
    pub async fn send(&self, data: &str) -> Result<String, TransportError> {
        let result = self.transport.client_command(data).await?;
        Ok(result)
    }
}
