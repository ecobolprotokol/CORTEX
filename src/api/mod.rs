pub mod routes;
pub mod auth;
pub mod handlers;

pub use routes::Router;
pub use auth::Authenticator;
pub use handlers::RequestHandler;

use crate::error::CortexError;

pub trait ApiServer {
    fn start(&self, bind: &str) -> Result<(), CortexError>;
    fn stop(&self) -> Result<(), CortexError>;
}

pub struct ApiManager {
    pub router: Router,
    pub authenticator: Authenticator,
    pub handler: RequestHandler,
    pub request_count: u64,
}

impl ApiManager {
    pub fn new(api_key: &str) -> Self {
        Self {
            router: Router::new(),
            authenticator: Authenticator::new(api_key),
            handler: RequestHandler::new(),
            request_count: 0,
        }
    }

    pub fn handle_request(
        &mut self,
        method: &str,
        path: &str,
        token: Option<&str>,
        body: Option<&str>,
    ) -> Result<String, CortexError> {
        if let Some(t) = token {
            self.authenticator.validate(t)?;
        }

        let endpoint = self.router.route(method, path)?;
        self.request_count += 1;

        match endpoint.as_str() {
            "inference" => {
                let input = body.unwrap_or("");
                self.handler.handle_inference(input)
            }
            "observe" => {
                let obs = body.unwrap_or("");
                self.handler.handle_observe(obs)
            }
            "query" => {
                let q = body.unwrap_or("");
                self.handler.handle_query(q)
            }
            "status" => self.handler.handle_status(),
            "verify" => {
                let claim = body.unwrap_or("");
                self.handler.handle_verify(claim)
            }
            "learn" => {
                let exp = body.unwrap_or("");
                self.handler.handle_learn(exp)
            }
            _ => Err(CortexError::RuntimeError(format!(
                "Unknown endpoint: {}",
                endpoint
            ))),
        }
    }

    pub fn request_count(&self) -> u64 {
        self.request_count
    }
}
