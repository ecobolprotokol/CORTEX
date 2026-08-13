pub mod routes;
pub mod auth;
pub mod handlers;

use crate::error::Result;

pub struct ApiServer {
    bind: String,
    api_key: Option<String>,
}

impl ApiServer {
    pub fn new(bind: &str, api_key: Option<String>) -> Self {
        Self {
            bind: bind.to_string(),
            api_key,
        }
    }

    pub async fn start(&self) -> Result<()> {
        tracing::info!("CORTEX API server listening on {}", self.bind);
        let listener = tokio::net::TcpListener::bind(&self.bind).await
            .map_err(|e| crate::error::CortexError::RuntimeError(format!("Failed to bind: {}", e)))?;
        loop {
            let (stream, _addr) = listener.accept().await
                .map_err(|e| crate::error::CortexError::RuntimeError(format!("Accept failed: {}", e)))?;
            let api_key = self.api_key.clone();
            tokio::spawn(async move {
                let io = hyper_util::rt::TokioIo::new(stream);
                let service = hyper::service::service_fn(move |req| {
                    let api_key = api_key.clone();
                    async move {
                        handlers::handle_request(req, &api_key).await
                    }
                });
                let _ = hyper_util::server::conn::auto::Builder::new(hyper_util::rt::TokioExecutor::new())
                    .serve_connection(io, service)
                    .await;
            });
        }
    }
}
