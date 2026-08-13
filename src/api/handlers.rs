use crate::cortex::CortexRuntime;
use crate::error::CortexError;
use crate::types::scalars::Scalar;

#[derive(Debug, Clone)]
pub struct HandlerResponse {
    pub status: u32,
    pub body: String,
    pub content_type: String,
}

pub struct RequestHandler {
    pub inference_count: u64,
    pub observe_count: u64,
    pub query_count: u64,
    pub verify_count: u64,
    pub learn_count: u64,
}

impl RequestHandler {
    pub fn new() -> Self {
        Self {
            inference_count: 0,
            observe_count: 0,
            query_count: 0,
            verify_count: 0,
            learn_count: 0,
        }
    }

    pub fn handle_inference(&mut self, input: &str) -> Result<String, CortexError> {
        if input.is_empty() {
            return Err(CortexError::InputError("Empty input".into()));
        }

        self.inference_count += 1;

        let response = format!(
            "{{\"status\": \"ok\", \"input\": \"{}\", \"inference_id\": {}}}",
            input.replace('"', "\\\""),
            self.inference_count
        );

        Ok(response)
    }

    pub fn handle_observe(&mut self, observation: &str) -> Result<String, CortexError> {
        if observation.is_empty() {
            return Err(CortexError::InputError("Empty observation".into()));
        }

        self.observe_count += 1;

        let response = format!(
            "{{\"status\": \"ok\", \"observation\": \"{}\", \"observe_id\": {}}}",
            observation.replace('"', "\\\""),
            self.observe_count
        );

        Ok(response)
    }

    pub fn handle_query(&mut self, query: &str) -> Result<String, CortexError> {
        if query.is_empty() {
            return Err(CortexError::InputError("Empty query".into()));
        }

        self.query_count += 1;

        let response = format!(
            "{{\"status\": \"ok\", \"query\": \"{}\", \"query_id\": {}}}",
            query.replace('"', "\\\""),
            self.query_count
        );

        Ok(response)
    }

    pub fn handle_status(&self) -> Result<String, CortexError> {
        let response = format!(
            "{{\"status\": \"ready\", \"version\": \"{}\", \"inference_count\": {}, \"observe_count\": {}, \"query_count\": {}, \"verify_count\": {}, \"learn_count\": {}}}",
            env!("CARGO_PKG_VERSION"),
            self.inference_count,
            self.observe_count,
            self.query_count,
            self.verify_count,
            self.learn_count
        );
        Ok(response)
    }

    pub fn handle_verify(&mut self, claim: &str) -> Result<String, CortexError> {
        if claim.is_empty() {
            return Err(CortexError::InputError("Empty claim".into()));
        }

        self.verify_count += 1;

        let confidence: Scalar = 0.5;
        let response = format!(
            "{{\"status\": \"ok\", \"claim\": \"{}\", \"confidence\": {:.2}, \"verify_id\": {}}}",
            claim.replace('"', "\\\""),
            confidence,
            self.verify_count
        );

        Ok(response)
    }

    pub fn handle_learn(&mut self, experience: &str) -> Result<String, CortexError> {
        if experience.is_empty() {
            return Err(CortexError::InputError("Empty experience".into()));
        }

        self.learn_count += 1;

        let response = format!(
            "{{\"status\": \"ok\", \"experience\": \"{}\", \"learn_id\": {}}}",
            experience.replace('"', "\\\""),
            self.learn_count
        );

        Ok(response)
    }

    pub fn total_requests(&self) -> u64 {
        self.inference_count
            + self.observe_count
            + self.query_count
            + self.verify_count
            + self.learn_count
    }

    pub fn reset_counts(&mut self) {
        self.inference_count = 0;
        self.observe_count = 0;
        self.query_count = 0;
        self.verify_count = 0;
        self.learn_count = 0;
    }
}

pub fn handle_inference_with_runtime(runtime: &mut CortexRuntime, input: &str) -> Result<String, CortexError> {
    let response = runtime.process(input)?;
    Ok(format!(
        "{{\"status\":\"ok\",\"input\":\"{}\",\"response\":\"{}\",\"episodes\":{},\"version\":{}}}",
        input.replace('"', "\\\""),
        response.replace('"', "\\\""),
        runtime.state.metadata.episode_count,
        runtime.state_version
    ))
}

pub fn handle_observe_with_runtime(runtime: &mut CortexRuntime, observation: &str) -> Result<String, CortexError> {
    let response = runtime.process(observation)?;
    Ok(format!(
        "{{\"status\":\"ok\",\"observation\":\"{}\",\"response\":\"{}\",\"episodes\":{},\"version\":{}}}",
        observation.replace('"', "\\\""),
        response.replace('"', "\\\""),
        runtime.state.metadata.episode_count,
        runtime.state_version
    ))
}

pub fn handle_query_with_runtime(runtime: &mut CortexRuntime, query: &str) -> Result<String, CortexError> {
    let response = runtime.process(query)?;
    Ok(format!(
        "{{\"status\":\"ok\",\"query\":\"{}\",\"response\":\"{}\",\"episodes\":{},\"version\":{}}}",
        query.replace('"', "\\\""),
        response.replace('"', "\\\""),
        runtime.state.metadata.episode_count,
        runtime.state_version
    ))
}

pub fn handle_status_with_runtime(runtime: &CortexRuntime) -> Result<String, CortexError> {
    Ok(format!(
        "{{\"status\":\"ok\",\"version\":\"{}\",\"state\":\"{:?}\",\"episodes\":{},\"learning_events\":{},\"vocabulary_size\":{},\"entities\":{},\"state_version\":{},\"mutations\":{}}}",
        env!("CARGO_PKG_VERSION"),
        runtime.runtime_state,
        runtime.state.metadata.episode_count,
        runtime.state.learning.total_learning_events,
        runtime.language_vocabulary.size(),
        runtime.state.world.entities.len(),
        runtime.state_version,
        runtime.mutation_log.records.len()
    ))
}

pub fn handle_checkpoint_with_runtime(runtime: &mut CortexRuntime) -> Result<String, CortexError> {
    runtime.save_state()?;
    Ok(format!(
        "{{\"status\":\"ok\",\"checkpoint_count\":{},\"episodes\":{},\"state_version\":{}}}",
        runtime.state.metadata.checkpoint_count,
        runtime.state.metadata.episode_count,
        runtime.state_version
    ))
}

impl Default for RequestHandler {
    fn default() -> Self {
        Self::new()
    }
}
