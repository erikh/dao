pub mod tcp;

use crate::protocol::{Instruction, Response};
use anyhow::Result;

pub trait Client {
    fn exchange(&mut self, instruction: Instruction) -> Result<Response>;
}

#[async_trait::async_trait]
pub trait AsyncClient {
    async fn exchange(&mut self, instruction: Instruction) -> Result<Response>;
}
