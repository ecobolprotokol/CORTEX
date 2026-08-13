pub mod routes;
pub mod auth;
pub mod handlers;

use crate::error::CortexError;

pub trait ApiServer {
    fn start(&self, bind: &str) -> Result<(), CortexError>;
    fn stop(&self) -> Result<(), CortexError>;
}
