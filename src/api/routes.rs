use crate::error::CortexError;

pub struct Router {
    routes: Vec<Route>,
    middleware: Vec<Middleware>,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub method: String,
    pub path: String,
    pub endpoint: String,
    pub requires_auth: bool,
}

#[derive(Debug, Clone)]
pub struct Middleware {
    pub name: String,
    pub enabled: bool,
}

impl Router {
    pub fn new() -> Self {
        let mut router = Self {
            routes: Vec::new(),
            middleware: Vec::new(),
        };

        router.register_default_routes();
        router
    }

    fn register_default_routes(&mut self) {
        self.routes.push(Route {
            method: "POST".into(),
            path: "/v1/inference".into(),
            endpoint: "inference".into(),
            requires_auth: true,
        });
        self.routes.push(Route {
            method: "POST".into(),
            path: "/v1/observe".into(),
            endpoint: "observe".into(),
            requires_auth: true,
        });
        self.routes.push(Route {
            method: "POST".into(),
            path: "/v1/query".into(),
            endpoint: "query".into(),
            requires_auth: true,
        });
        self.routes.push(Route {
            method: "GET".into(),
            path: "/v1/status".into(),
            endpoint: "status".into(),
            requires_auth: false,
        });
        self.routes.push(Route {
            method: "POST".into(),
            path: "/v1/verify".into(),
            endpoint: "verify".into(),
            requires_auth: true,
        });
        self.routes.push(Route {
            method: "POST".into(),
            path: "/v1/learn".into(),
            endpoint: "learn".into(),
            requires_auth: true,
        });
        self.routes.push(Route {
            method: "POST".into(),
            path: "/v1/checkpoint".into(),
            endpoint: "checkpoint".into(),
            requires_auth: true,
        });
        self.routes.push(Route {
            method: "GET".into(),
            path: "/v1/inspect".into(),
            endpoint: "inspect".into(),
            requires_auth: true,
        });
        self.routes.push(Route {
            method: "GET".into(),
            path: "/health".into(),
            endpoint: "health".into(),
            requires_auth: false,
        });

        self.middleware.push(Middleware {
            name: "logging".into(),
            enabled: true,
        });
        self.middleware.push(Middleware {
            name: "rate_limit".into(),
            enabled: true,
        });
    }

    pub fn route(&self, method: &str, path: &str) -> Result<String, CortexError> {
        for route in &self.routes {
            if route.method == method && route.path == path {
                return Ok(route.endpoint.clone());
            }
        }

        Err(CortexError::RuntimeError(format!(
            "No route for {} {}",
            method, path
        )))
    }

    pub fn requires_auth(&self, method: &str, path: &str) -> bool {
        self.routes
            .iter()
            .any(|r| r.method == method && r.path == path && r.requires_auth)
    }

    pub fn register_route(
        &mut self,
        method: &str,
        path: &str,
        endpoint: &str,
        requires_auth: bool,
    ) {
        self.routes.push(Route {
            method: method.into(),
            path: path.into(),
            endpoint: endpoint.into(),
            requires_auth,
        });
    }

    pub fn list_routes(&self) -> Vec<(&str, &str, &str, bool)> {
        self.routes
            .iter()
            .map(|r| {
                (
                    r.method.as_str(),
                    r.path.as_str(),
                    r.endpoint.as_str(),
                    r.requires_auth,
                )
            })
            .collect()
    }

    pub fn enable_middleware(&mut self, name: &str) {
        if let Some(mw) = self.middleware.iter_mut().find(|m| m.name == name) {
            mw.enabled = true;
        }
    }

    pub fn disable_middleware(&mut self, name: &str) {
        if let Some(mw) = self.middleware.iter_mut().find(|m| m.name == name) {
            mw.enabled = false;
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
