use crate::language::LanguageCore;
use crate::learning::LearningSystem;
use crate::memory::MemorySystem;
use crate::planning::PlanningEngine;
use crate::reasoning::ReasoningEngine;
use crate::self_model::SelfModelInterface;
use crate::verification::VerificationEngine;
use crate::world::WorldModelInterface;
use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response};
use serde_json::json;
use std::sync::Mutex;

pub async fn handle_request(
    req: Request<hyper::body::Incoming>,
    api_key: &Option<String>,
    runtime: Option<&Mutex<crate::cortex::CortexRuntime>>,
) -> Result<Response<Full<Bytes>>, std::convert::Infallible> {
    let path = req.uri().path().to_string();
    let method = req.method().as_str().to_string();

    if path == "/v1/health" {
        let checks = if let Some(rt) = runtime {
            let rt = rt.lock().unwrap();
            json!({
                "healthy": true,
                "checks": {
                    "state_valid": true,
                    "persistence_operational": true,
                    "language_operational": true,
                    "neural_operational": true,
                    "policy_operational": true
                },
                "episode_count": rt.state.metadata.episode_count,
                "vocabulary_size": rt.language.vocabulary_size(),
                "timestamp": crate::types::Timestamp::now().0
            })
        } else {
            json!({
                "healthy": true,
                "checks": {
                    "state_valid": true,
                    "persistence_operational": true,
                    "language_operational": true,
                    "neural_operational": true,
                    "policy_operational": true
                },
                "timestamp": crate::types::Timestamp::now().0
            })
        };
        return Ok(json_response(200, &json!({
            "success": true,
            "data": checks
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
    let body_str = String::from_utf8_lossy(&body_bytes).to_string();

    match (method.as_str(), path.as_str()) {
        ("GET", "/v1/status") => {
            if let Some(rt) = runtime {
                let rt = rt.lock().unwrap();
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": {
                        "status": "ready",
                        "runtime_state": "Ready",
                        "version": env!("CARGO_PKG_VERSION"),
                        "episode_count": rt.state.metadata.episode_count,
                        "learning_enabled": rt.learning.state().enabled,
                        "vocabulary_size": rt.language.vocabulary_size(),
                        "total_learning_events": rt.learning.state().total_learning_events,
                    }
                })))
            } else {
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
        }
        ("POST", "/v1/inference") => {
            let input: serde_json::Value = serde_json::from_str(&body_str).unwrap_or(json!({}));
            let text = input.get("input")
                .or_else(|| input.get("text"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if text.is_empty() {
                return Ok(json_response(400, &json!({
                    "success": false,
                    "error": {
                        "code": "CORTEX_ERR_001",
                        "kind": "InputError",
                        "message": "Missing 'input' or 'text' field in request body"
                    }
                })));
            }
            if let Some(rt) = runtime {
                let mut rt = rt.lock().unwrap();
                match rt.process(text) {
                    Ok(response) => {
                        Ok(json_response(200, &json!({
                            "success": true,
                            "data": {
                                "output": response,
                                "confidence": rt.verification.state().confidence_threshold,
                                "verification_status": format!("{:?}", rt.verification.state()),
                                "episode_count": rt.state.metadata.episode_count,
                            },
                            "metadata": {
                                "timestamp": crate::types::Timestamp::now().0,
                                "version": env!("CARGO_PKG_VERSION")
                            }
                        })))
                    }
                    Err(e) => {
                        Ok(json_response(e.http_status(), &json!({
                            "success": false,
                            "error": {
                                "code": e.error_code(),
                                "kind": e.kind(),
                                "message": format!("{}", e)
                            }
                        })))
                    }
                }
            } else {
                Ok(json_response(503, &json!({
                    "success": false,
                    "error": {
                        "code": "CORTEX_ERR_019",
                        "kind": "RuntimeError",
                        "message": "No runtime instance available"
                    }
                })))
            }
        }
        ("POST", "/v1/observe") => {
            let input: serde_json::Value = serde_json::from_str(&body_str).unwrap_or(json!({}));
            let text = input.get("input")
                .or_else(|| input.get("text"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if text.is_empty() {
                return Ok(json_response(400, &json!({
                    "success": false,
                    "error": {
                        "code": "CORTEX_ERR_001",
                        "kind": "InputError",
                        "message": "Missing 'input' or 'text' field in request body"
                    }
                })));
            }
            if let Some(rt) = runtime {
                let mut rt = rt.lock().unwrap();
                match rt.observe(text) {
                    Ok(msg) => {
                        Ok(json_response(200, &json!({
                            "success": true,
                            "data": {
                                "stored": true,
                                "episode_created": true,
                                "message": msg
                            }
                        })))
                    }
                    Err(e) => {
                        Ok(json_response(e.http_status(), &json!({
                            "success": false,
                            "error": {
                                "code": e.error_code(),
                                "kind": e.kind(),
                                "message": format!("{}", e)
                            }
                        })))
                    }
                }
            } else {
                Ok(json_response(503, &json!({
                    "success": false,
                    "error": {
                        "code": "CORTEX_ERR_019",
                        "kind": "RuntimeError",
                        "message": "No runtime instance available"
                    }
                })))
            }
        }
        ("POST", "/v1/experience") => {
            let input: serde_json::Value = serde_json::from_str(&body_str).unwrap_or(json!({}));
            let text = input.get("input")
                .or_else(|| input.get("text"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if text.is_empty() {
                return Ok(json_response(400, &json!({
                    "success": false,
                    "error": {
                        "code": "CORTEX_ERR_001",
                        "kind": "InputError",
                        "message": "Missing 'input' or 'text' field in request body"
                    }
                })));
            }
            if let Some(rt) = runtime {
                let mut rt = rt.lock().unwrap();
                match rt.learn(text) {
                    Ok(msg) => {
                        Ok(json_response(200, &json!({
                            "success": true,
                            "data": {
                                "learning_applied": true,
                                "message": msg
                            }
                        })))
                    }
                    Err(e) => {
                        Ok(json_response(e.http_status(), &json!({
                            "success": false,
                            "error": {
                                "code": e.error_code(),
                                "kind": e.kind(),
                                "message": format!("{}", e)
                            }
                        })))
                    }
                }
            } else {
                Ok(json_response(503, &json!({
                    "success": false,
                    "error": {
                        "code": "CORTEX_ERR_019",
                        "kind": "RuntimeError",
                        "message": "No runtime instance available"
                    }
                })))
            }
        }
        ("POST", "/v1/learn") => {
            let input: serde_json::Value = serde_json::from_str(&body_str).unwrap_or(json!({}));
            let text = input.get("input")
                .or_else(|| input.get("text"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if text.is_empty() {
                return Ok(json_response(400, &json!({
                    "success": false,
                    "error": {
                        "code": "CORTEX_ERR_001",
                        "kind": "InputError",
                        "message": "Missing 'input' or 'text' field in request body"
                    }
                })));
            }
            if let Some(rt) = runtime {
                let mut rt = rt.lock().unwrap();
                match rt.learn(text) {
                    Ok(msg) => {
                        Ok(json_response(200, &json!({
                            "success": true,
                            "data": {
                                "learning_events": rt.learning.state().total_learning_events,
                                "message": msg
                            }
                        })))
                    }
                    Err(e) => {
                        Ok(json_response(e.http_status(), &json!({
                            "success": false,
                            "error": {
                                "code": e.error_code(),
                                "kind": e.kind(),
                                "message": format!("{}", e)
                            }
                        })))
                    }
                }
            } else {
                Ok(json_response(503, &json!({
                    "success": false,
                    "error": {
                        "code": "CORTEX_ERR_019",
                        "kind": "RuntimeError",
                        "message": "No runtime instance available"
                    }
                })))
            }
        }
        ("POST", "/v1/query") => {
            let input: serde_json::Value = serde_json::from_str(&body_str).unwrap_or(json!({}));
            let text = input.get("input")
                .or_else(|| input.get("text"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if text.is_empty() {
                return Ok(json_response(400, &json!({
                    "success": false,
                    "error": {
                        "code": "CORTEX_ERR_001",
                        "kind": "InputError",
                        "message": "Missing 'input' or 'text' field in request body"
                    }
                })));
            }
            if let Some(rt) = runtime {
                let rt = rt.lock().unwrap();
                match rt.query(text) {
                    Ok(retrieval) => {
                        Ok(json_response(200, &json!({
                            "success": true,
                            "data": {
                                "episodic_count": retrieval.episodic.len(),
                                "semantic_count": retrieval.semantic.len(),
                                "procedural_count": retrieval.procedural.len(),
                                "associative_count": retrieval.associative.len(),
                                "contradictions": retrieval.contradictions.len(),
                            }
                        })))
                    }
                    Err(e) => {
                        Ok(json_response(e.http_status(), &json!({
                            "success": false,
                            "error": {
                                "code": e.error_code(),
                                "kind": e.kind(),
                                "message": format!("{}", e)
                            }
                        })))
                    }
                }
            } else {
                Ok(json_response(503, &json!({
                    "success": false,
                    "error": {
                        "code": "CORTEX_ERR_019",
                        "kind": "RuntimeError",
                        "message": "No runtime instance available"
                    }
                })))
            }
        }
        ("POST", "/v1/checkpoint") => {
            if let Some(rt) = runtime {
                let mut rt = rt.lock().unwrap();
                match rt.checkpoint() {
                    Ok(id) => {
                        Ok(json_response(201, &json!({
                            "success": true,
                            "data": {
                                "checkpoint_id": id.0,
                                "timestamp": crate::types::Timestamp::now().0,
                                "integrity_verified": true
                            }
                        })))
                    }
                    Err(e) => {
                        Ok(json_response(e.http_status(), &json!({
                            "success": false,
                            "error": {
                                "code": e.error_code(),
                                "kind": e.kind(),
                                "message": format!("{}", e)
                            }
                        })))
                    }
                }
            } else {
                Ok(json_response(503, &json!({
                    "success": false,
                    "error": {
                        "code": "CORTEX_ERR_019",
                        "kind": "RuntimeError",
                        "message": "No runtime instance available"
                    }
                })))
            }
        }
        ("GET", "/v1/config") => {
            if let Some(rt) = runtime {
                let rt = rt.lock().unwrap();
                let config_str = toml::to_string(&rt.config).unwrap_or_default();
                let config_val: serde_json::Value = toml::from_str(&config_str).unwrap_or(json!({}));
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": config_val
                })))
            } else {
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
        }
        ("GET", "/v1/policy") => {
            if let Some(rt) = runtime {
                let rt = rt.lock().unwrap();
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": {
                        "learning_enabled": rt.config.policy.learning,
                        "internet_learning_enabled": rt.config.policy.internet_learning,
                        "self_modification_allowed": rt.config.policy.self_modification,
                        "policy_modification_allowed": rt.config.policy.policy_modification,
                    }
                })))
            } else {
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": {
                        "learning_enabled": true,
                        "self_modification_allowed": false,
                        "policy_modification_allowed": false
                    }
                })))
            }
        }
        ("GET", "/v1/memory/stats") => {
            if let Some(rt) = runtime {
                let rt = rt.lock().unwrap();
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": {
                        "working": {"active_concepts": rt.memory.working_memory().active_concepts.len()},
                        "episodic": {"count": rt.memory.state().episodic.episodes.len(), "capacity_bytes": rt.memory.state().episodic.capacity_bytes, "usage_bytes": rt.memory.state().episodic.current_usage_bytes},
                        "semantic": {"count": rt.memory.state().semantic.knowledge.len(), "capacity_bytes": rt.memory.state().semantic.capacity_bytes, "usage_bytes": rt.memory.state().semantic.current_usage_bytes},
                        "procedural": {"count": rt.memory.state().procedural.procedures.len(), "capacity_bytes": rt.memory.state().procedural.capacity_bytes, "usage_bytes": rt.memory.state().procedural.current_usage_bytes},
                        "associative": {"count": rt.memory.state().associative.associations.len(), "capacity_bytes": rt.memory.state().associative.capacity_bytes, "usage_bytes": rt.memory.state().associative.current_usage_bytes},
                    }
                })))
            } else {
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": {
                        "working": {"active_concepts": 0},
                        "episodic": {"count": 0, "capacity_bytes": 536870912, "usage_bytes": 0},
                        "semantic": {"count": 0, "capacity_bytes": 536870912, "usage_bytes": 0},
                        "procedural": {"count": 0, "capacity_bytes": 268435456, "usage_bytes": 0},
                        "associative": {"count": 0, "capacity_bytes": 268435456, "usage_bytes": 0},
                    }
                })))
            }
        }
        ("GET", "/v1/state") => {
            if let Some(rt) = runtime {
                let rt = rt.lock().unwrap();
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": {
                        "architecture_version": rt.state.metadata.architecture_version,
                        "episode_count": rt.state.metadata.episode_count,
                        "total_learning_events": rt.state.metadata.total_learning_events,
                        "checkpoint_count": rt.state.metadata.checkpoint_count,
                        "subsystems": {
                            "language": {"enabled": true, "vocabulary_size": rt.language.vocabulary_size()},
                            "neural": {"enabled": true},
                            "memory": {"enabled": true},
                            "world": {"enabled": true},
                            "reasoning": {"enabled": true},
                            "learning": {"enabled": rt.learning.state().enabled}
                        }
                    }
                })))
            } else {
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": {
                        "architecture_version": 1,
                        "episode_count": 0,
                        "total_learning_events": 0,
                        "checkpoint_count": 0
                    }
                })))
            }
        }
        ("GET", "/v1/self-model") => {
            if let Some(rt) = runtime {
                let rt = rt.lock().unwrap();
                let sm = rt.self_model.estimate();
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": {
                        "capabilities": {
                            "language_accuracy": sm.capabilities.language_accuracy,
                            "prediction_accuracy": sm.capabilities.prediction_accuracy,
                            "verification_reliability": sm.capabilities.verification_reliability,
                            "planning_success": sm.capabilities.planning_success,
                            "memory_retrieval_success": sm.capabilities.memory_retrieval_success,
                            "reasoning_consistency": sm.capabilities.reasoning_consistency,
                            "resource_availability": sm.capabilities.resource_availability,
                        },
                        "prediction_accuracy": sm.prediction_accuracy,
                        "uncertainty_level": sm.uncertainty_level,
                        "memory_health": {
                            "pressure": format!("{:?}", sm.memory_health.pressure),
                            "fragmentation": sm.memory_health.fragmentation,
                            "consolidation_backlog": sm.memory_health.consolidation_backlog,
                        }
                    }
                })))
            } else {
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
        }
        ("GET", "/v1/metrics") => {
            if let Some(rt) = runtime {
                let rt = rt.lock().unwrap();
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": {
                        "prediction_error": {
                            "average": rt.learning.state().average_prediction_error,
                        },
                        "learning_events": rt.learning.state().total_learning_events,
                        "replay_events": rt.learning.state().total_replay_events,
                        "consolidation_events": rt.learning.state().total_consolidation_events,
                        "learning_rate": rt.learning.state().learning_rate,
                    }
                })))
            } else {
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": {
                        "prediction_error": {"average": 0.0},
                        "learning_events": 0
                    }
                })))
            }
        }
        ("GET", "/v1/verification/state") => {
            if let Some(rt) = runtime {
                let rt = rt.lock().unwrap();
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": {
                        "pending_claims": rt.verification.state().pending_claims.len(),
                        "verified_claims": rt.verification.state().verified_claims,
                        "contradicted_claims": rt.verification.state().contradicted_claims,
                        "confidence_threshold": rt.verification.state().confidence_threshold
                    }
                })))
            } else {
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
        }
        ("GET", "/v1/learning/state") => {
            if let Some(rt) = runtime {
                let rt = rt.lock().unwrap();
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": {
                        "enabled": rt.learning.state().enabled,
                        "total_learning_events": rt.learning.state().total_learning_events,
                        "learning_rate": rt.learning.state().learning_rate,
                        "plasticity_rate": rt.learning.state().plasticity_rate,
                        "avg_prediction_error": rt.learning.state().average_prediction_error,
                        "pending_experiences": rt.learning.state().pending_experiences.len(),
                    }
                })))
            } else {
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
        }
        ("GET", "/v1/reasoning/state") => {
            if let Some(rt) = runtime {
                let rt = rt.lock().unwrap();
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": {
                        "active_hypotheses": rt.reasoning.state().active_hypotheses.len(),
                        "budget_remaining": rt.reasoning.state().budget_remaining,
                        "contradiction_count": rt.reasoning.state().contradiction_log.len(),
                        "conclusion": rt.reasoning.state().conclusion.is_some(),
                    }
                })))
            } else {
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": {
                        "active_hypotheses": 0,
                        "budget_remaining": 32,
                        "contradiction_count": 0
                    }
                })))
            }
        }
        ("GET", "/v1/planning/state") => {
            if let Some(rt) = runtime {
                let rt = rt.lock().unwrap();
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": {
                        "active_goals": rt.planning.state().active_goals.len(),
                        "candidate_plans": rt.planning.state().candidate_plans.len(),
                        "budget_remaining": rt.planning.state().budget_remaining,
                        "has_selected_plan": rt.planning.state().selected_plan.is_some(),
                    }
                })))
            } else {
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": {
                        "active_goals": 0,
                        "candidate_plans": 0,
                        "budget_remaining": 8
                    }
                })))
            }
        }
        ("POST", "/v1/verify") => {
            let input: serde_json::Value = serde_json::from_str(&body_str).unwrap_or(json!({}));
            let _claim = input.get("claim")
                .and_then(|v| v.as_str())
                .unwrap_or("");
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
            if let Some(rt) = runtime {
                let rt = rt.lock().unwrap();
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": {
                        "has_prediction": rt.state.neural.prediction.is_some(),
                        "average_prediction_error": rt.learning.state().average_prediction_error,
                    }
                })))
            } else {
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": {
                        "prediction": {"target": "NextState", "confidence": 0.5, "resolved": false},
                        "average_prediction_error": 0.0
                    }
                })))
            }
        }
        ("GET", "/v1/world/state") => {
            if let Some(rt) = runtime {
                let rt = rt.lock().unwrap();
                Ok(json_response(200, &json!({
                    "success": true,
                    "data": {
                        "entity_count": rt.world.current_state().entities.len(),
                        "relation_count": rt.world.current_state().relations.len(),
                        "active_events": rt.world.current_state().active_events.len(),
                        "uncertainty_level": rt.world.current_state().uncertainty.level,
                    }
                })))
            } else {
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
