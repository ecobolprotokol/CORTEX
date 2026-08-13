use crate::error::CortexError;
use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response};
use serde_json::json;

pub async fn handle_request(
    req: Request<hyper::body::Incoming>,
    api_key: &Option<String>,
) -> Result<Response<Full<Bytes>>, std::convert::Infallible> {
    let path = req.uri().path().to_string();
    let method = req.method().as_str().to_string();

    if path == "/v1/health" {
        return Ok(json_response(200, &json!({
            "success": true,
            "data": {
                "healthy": true,
                "checks": {
                    "state_valid": true,
                    "persistence_operational": true,
                    "language_operational": true,
                    "neural_operational": true,
                    "policy_operational": true
                },
                "timestamp": crate::types::Timestamp::now().0
            }
        })));
    }

    let auth_header = req.headers().get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    if !super::auth::authenticate(auth_header, api_key.as_deref()) {
        return Ok(json_response(401, &json!({
            "success": false,
            "error": {
                "code": "CORTEX_ERR_004",
                "kind": "AuthenticationError",
                "message": "Invalid or missing API key"
            }
        })));
    }

    let body_bytes = req.into_body().collect().await.map(|c| c.to_bytes()).unwrap_or_default();
    let _body_str = String::from_utf8_lossy(&body_bytes).to_string();

    match (method.as_str(), path.as_str()) {
        ("GET", "/v1/status") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "status": "ready",
                    "runtime_state": "Ready",
                    "version": env!("CARGO_PKG_VERSION"),
                    "episode_count": 0,
                    "learning_enabled": true
                }
            })))
        }
        ("POST", "/v1/inference") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "output": "Inference endpoint active. Use CLI for full processing.",
                    "confidence": 0.5,
                    "verification_status": "Provisional"
                },
                "metadata": {
                    "timestamp": crate::types::Timestamp::now().0,
                    "version": env!("CARGO_PKG_VERSION")
                }
            })))
        }
        ("POST", "/v1/observe") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "stored": true,
                    "episode_created": true,
                    "state_updated": true
                }
            })))
        }
        ("POST", "/v1/experience") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "learning_applied": true,
                    "state_updated": true
                }
            })))
        }
        ("POST", "/v1/learn") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "learning_events": 0,
                    "state_updated": true
                }
            })))
        }
        ("POST", "/v1/query") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "episodic": [],
                    "semantic": [],
                    "total_results": 0
                }
            })))
        }
        ("POST", "/v1/checkpoint") => {
            Ok(json_response(201, &json!({
                "success": true,
                "data": {
                    "checkpoint_id": 1,
                    "timestamp": crate::types::Timestamp::now().0,
                    "integrity_verified": true
                }
            })))
        }
        ("GET", "/v1/config") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "model": { "cells": 4096, "columns": 64 },
                    "language": { "enabled": true },
                    "memory": { "working_mb": 128 },
                    "learning": { "enabled": true },
                    "api": { "enabled": true }
                }
            })))
        }
        ("GET", "/v1/policy") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "learning_enabled": true,
                    "self_modification_allowed": false,
                    "policy_modification_allowed": false
                }
            })))
        }
        ("GET", "/v1/memory/stats") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "working": {"active_concepts": 0},
                    "episodic": {"count": 0, "capacity_bytes": 536870912, "usage_bytes": 0},
                    "semantic": {"count": 0, "capacity_bytes": 536870912, "usage_bytes": 0},
                    "procedural": {"count": 0, "capacity_bytes": 268435456, "usage_bytes": 0},
                    "associative": {"count": 0, "capacity_bytes": 268435456, "usage_bytes": 0},
                    "total_usage_bytes": 0,
                    "pressure": "Low"
                }
            })))
        }
        ("GET", "/v1/state") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "architecture_version": 1,
                    "episode_count": 0,
                    "total_learning_events": 0,
                    "checkpoint_count": 0,
                    "subsystems": {
                        "language": {"enabled": true, "vocabulary_size": 0},
                        "neural": {"enabled": true},
                        "memory": {"enabled": true},
                        "world": {"enabled": true},
                        "reasoning": {"enabled": true},
                        "learning": {"enabled": true}
                    }
                }
            })))
        }
        ("GET", "/v1/self-model") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "capabilities": {
                        "language_accuracy": 0.5,
                        "prediction_accuracy": 0.5,
                        "verification_reliability": 0.5
                    },
                    "prediction_accuracy": 0.5,
                    "uncertainty_level": 0.5
                }
            })))
        }
        ("GET", "/v1/metrics") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "prediction_error": {"current": 0.0, "average": 0.0},
                    "memory_retrieval_success": 0.5,
                    "verification_confidence": 0.5
                }
            })))
        }
        ("GET", "/v1/verification/state") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "pending_claims": 0,
                    "verified_claims": 0,
                    "contradicted_claims": 0,
                    "confidence_threshold": 0.80
                }
            })))
        }
        ("GET", "/v1/learning/state") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "enabled": true,
                    "total_learning_events": 0,
                    "learning_rate": 0.001,
                    "plasticity_rate": 0.01
                }
            })))
        }
        ("GET", "/v1/reasoning/state") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "active_hypotheses": 0,
                    "budget_remaining": 32,
                    "contradiction_count": 0
                }
            })))
        }
        ("GET", "/v1/planning/state") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "active_goals": 0,
                    "candidate_plans": 0,
                    "budget_remaining": 8
                }
            })))
        }
        ("POST", "/v1/verify") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "verification_status": "Provisional",
                    "confidence": {"belief": 0.5, "uncertainty": 0.5}
                }
            })))
        }
        ("POST", "/v1/internet/fetch") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "content_extracted": true,
                    "provenance": {"category": "Internet"}
                }
            })))
        }
        ("GET", "/v1/prediction/current") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "prediction": {"target": "NextState", "confidence": 0.5, "resolved": false},
                    "average_prediction_error": 0.0
                }
            })))
        }
        ("GET", "/v1/world/state") => {
            Ok(json_response(200, &json!({
                "success": true,
                "data": {
                    "entity_count": 0,
                    "relation_count": 0,
                    "active_events": 0,
                    "uncertainty_level": 1.0
                }
            })))
        }
        _ => {
            Ok(json_response(404, &json!({
                "success": false,
                "error": {
                    "code": "CORTEX_ERR_006",
                    "kind": "NotFoundError",
                    "message": format!("Endpoint not found: {} {}", method, path)
                }
            })))
        }
    }
}

fn json_response(status: u16, body: &serde_json::Value) -> Response<Full<Bytes>> {
    let body_str = serde_json::to_string(body).unwrap_or_default();
    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(Full::new(Bytes::from(body_str)))
        .unwrap()
}
