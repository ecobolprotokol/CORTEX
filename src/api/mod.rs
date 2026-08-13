pub mod routes;
pub mod auth;
pub mod handlers;

use crate::error::Result;
use std::sync::{Arc, Mutex};

pub struct ApiServer {
    bind: String,
    api_key: Option<String>,
    runtime: Option<Arc<Mutex<crate::cortex::CortexRuntime>>>,
}

impl ApiServer {
    pub fn new(bind: &str, api_key: Option<String>) -> Self {
        Self {
            bind: bind.to_string(),
            api_key,
            runtime: None,
        }
    }

    pub fn with_runtime(mut self, runtime: Arc<Mutex<crate::cortex::CortexRuntime>>) -> Self {
        self.runtime = Some(runtime);
        self
    }

    pub async fn start(&self) -> Result<()> {
        let runtime = self.runtime.clone();
        let api_key = self.api_key.clone();
        tracing::info!("CORTEX API server listening on {}", self.bind);
        let listener = tokio::net::TcpListener::bind(&self.bind).await
            .map_err(|e| crate::error::CortexError::RuntimeError(format!("Failed to bind: {}", e)))?;
        loop {
            let (stream, _addr) = listener.accept().await
                .map_err(|e| crate::error::CortexError::RuntimeError(format!("Accept failed: {}", e)))?;
            let api_key = api_key.clone();
            let runtime = runtime.clone();
            tokio::spawn(async move {
                let io = hyper_util::rt::TokioIo::new(stream);
                let service = hyper::service::service_fn(move |req| {
                    let api_key = api_key.clone();
                    let runtime = runtime.clone();
                    async move {
                        handlers::handle_request(req, &api_key, runtime.as_deref()).await
                    }
                });
                let _ = hyper_util::server::conn::auto::Builder::new(hyper_util::rt::TokioExecutor::new())
                    .serve_connection(io, service)
                    .await;
            });
        }
    }
}
