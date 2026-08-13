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

    pub fn start_synchronous_server(
        &mut self,
        bind: &str,
    ) -> Result<(), CortexError> {
        use std::io::{BufRead, BufReader, Write};
        use std::net::TcpListener;

        let listener = TcpListener::bind(bind)
            .map_err(|e| CortexError::RuntimeError(format!("Failed to bind {}: {}", bind, e)))?;

        tracing::info!(bind = %bind, "API server listening");

        listener.set_nonblocking(false)
            .map_err(|e| CortexError::RuntimeError(format!("Failed to set nonblocking: {}", e)))?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let peer = stream.peer_addr().map(|a| a.to_string()).unwrap_or_default();
                    let reader = BufReader::new(&stream);
                    let mut writer = &stream;

                    let mut _request_line = String::new();
                    let mut headers = Vec::new();
                    let mut content_length = 0usize;
                    let mut auth_token: Option<String> = None;

                    let mut lines = reader.lines();
                    if let Some(Ok(first)) = lines.next() {
                        _request_line = first;
                    } else {
                        continue;
                    }

                    for line in lines {
                        match line {
                            Ok(line) if line.is_empty() => break,
                            Ok(line) => {
                                if let Some(val) = line.strip_prefix("Content-Length: ") {
                                    content_length = val.trim().parse().unwrap_or(0);
                                }
                                if let Some(val) = line.strip_prefix("Authorization: ") {
                                    auth_token = Some(val.trim().to_string());
                                }
                                headers.push(line);
                            }
                            Err(_) => break,
                        }
                    }

                    let mut body = vec![0u8; content_length];
                    if content_length > 0 {
                        use std::io::Read;
                        let _ = std::io::Read::read(&mut (&stream), &mut body);
                    }

                    let body_str = String::from_utf8_lossy(&body).to_string();
                    let parts: Vec<&str> = request_line.split_whitespace().collect();
                    let method = parts.first().unwrap_or(&"");
                    let path = parts.get(1).unwrap_or(&"/");

                    tracing::debug!(peer = %peer, method = %method, path = %path, "Request received");

                    let response = match self.handle_request(
                        method,
                        path,
                        auth_token.as_deref(),
                        Some(&body_str),
                    ) {
                        Ok(body) => format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            body.len(),
                            body
                        ),
                        Err(e) => {
                            let (status, msg) = match &e {
                                CortexError::PolicyError(_) => (403, format!("{}", e)),
                                CortexError::InputError(_) => (400, format!("{}", e)),
                                _ => (500, format!("{}", e)),
                            };
                            format!(
                                "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{{\"error\":\"{}\"}}",
                                status,
                                if status == 403 { "Forbidden" } else if status == 400 { "Bad Request" } else { "Internal Server Error" },
                                msg.len() + 12,
                                msg.replace('"', "\\\"")
                            )
                        }
                    };

                    let _ = writer.write_all(response.as_bytes());
                    tracing::debug!(peer = %peer, "Response sent");
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Accept failed");
                }
            }
        }
        Ok(())
    }
}
