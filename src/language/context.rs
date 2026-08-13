use std::collections::HashMap;

use crate::types::common::ContextState;
use crate::types::ids::{ConceptId, EpisodeId, SymbolId};

#[derive(Debug, Clone, Default)]
pub struct HierarchicalContext {
    pub symbol_context: Vec<SymbolId>,
    pub sentence_context: Vec<String>,
    pub conversation_context: Vec<String>,
    pub episode_context: Vec<EpisodeId>,
    pub semantic_context: Vec<ConceptId>,
    pub world_context: Vec<String>,
    pub long_term_context: Vec<String>,
}

pub struct ContextModel {
    pub window_sizes: WindowSizes,
    pub context: HierarchicalContext,
    pub attention_weights: HashMap<String, f32>,
}

#[derive(Debug, Clone)]
pub struct WindowSizes {
    pub symbol: usize,
    pub sentence: usize,
    pub conversation: usize,
    pub episode: usize,
    pub semantic: usize,
    pub world: usize,
    pub long_term: usize,
}

impl Default for WindowSizes {
    fn default() -> Self {
        Self {
            symbol: 16,
            sentence: 8,
            conversation: 32,
            episode: 16,
            semantic: 24,
            world: 12,
            long_term: 64,
        }
    }
}

impl ContextModel {
    pub fn new(window_size: u32) -> Self {
        Self {
            window_sizes: WindowSizes {
                symbol: window_size as usize,
                sentence: (window_size / 2).max(1) as usize,
                conversation: window_size as usize,
                episode: (window_size / 2).max(1) as usize,
                semantic: window_size as usize,
                world: (window_size / 2).max(1) as usize,
                long_term: (window_size * 2) as usize,
            },
            context: HierarchicalContext::default(),
            attention_weights: HashMap::new(),
        }
    }

    pub fn add_concept(&mut self, concept: ConceptId) {
        if self.context.semantic_context.len() >= self.window_sizes.semantic {
            self.context.semantic_context.remove(0);
        }
        self.context.semantic_context.push(concept);
    }

    pub fn assemble(
        &mut self,
        working_memory: &crate::types::state::WorkingMemory,
        global_context: &ContextState,
    ) -> HierarchicalContext {
        self.update_symbol_context(working_memory);
        self.update_sentence_context(working_memory);
        self.update_conversation_context(working_memory);
        self.update_episode_context(global_context);
        self.update_semantic_context(global_context);
        self.update_world_context(working_memory, global_context);
        self.update_long_term_context();

        self.compute_attention_weights();

        self.context.clone()
    }

    fn update_symbol_context(&mut self, wm: &crate::types::state::WorkingMemory) {
        for concept in &wm.active_concepts {
            let symbol_id = SymbolId::from(concept.raw());
            if self.context.symbol_context.len() >= self.window_sizes.symbol {
                self.context.symbol_context.remove(0);
            }
            self.context.symbol_context.push(symbol_id);
        }
    }

    fn update_sentence_context(&mut self, wm: &crate::types::state::WorkingMemory) {
        for input in wm
            .recent_inputs
            .iter()
            .rev()
            .take(self.window_sizes.sentence)
        {
            self.context.sentence_context.push(input.clone());
        }
        if self.context.sentence_context.len() > self.window_sizes.sentence {
            self.context
                .sentence_context
                .truncate(self.window_sizes.sentence);
        }
    }

    fn update_conversation_context(&mut self, wm: &crate::types::state::WorkingMemory) {
        if let Some(input) = &wm.input {
            self.context.conversation_context.push(input.clone());
        }
        for output in &wm.recent_outputs {
            self.context.conversation_context.push(output.clone());
        }
        if self.context.conversation_context.len() > self.window_sizes.conversation {
            let excess = self.context.conversation_context.len() - self.window_sizes.conversation;
            self.context.conversation_context.drain(..excess);
        }
    }

    fn update_episode_context(&mut self, global: &ContextState) {
        self.context.episode_context = global
            .episode_context
            .iter()
            .take(self.window_sizes.episode)
            .cloned()
            .collect();
    }

    fn update_semantic_context(&mut self, global: &ContextState) {
        for concept in &global.active_concepts {
            if !self.context.semantic_context.contains(concept) {
                if self.context.semantic_context.len() >= self.window_sizes.semantic {
                    self.context.semantic_context.remove(0);
                }
                self.context.semantic_context.push(*concept);
            }
        }
    }

    fn update_world_context(
        &mut self,
        wm: &crate::types::state::WorkingMemory,
        global: &ContextState,
    ) {
        self.context.world_context.clear();
        for entity in &global.world_assumptions {
            self.context
                .world_context
                .push(format!("entity:{}", entity.raw()));
        }
        for assumption in &wm.world_assumptions {
            self.context
                .world_context
                .push(format!("assumption:{}", assumption.raw()));
        }
        if self.context.world_context.len() > self.window_sizes.world {
            self.context.world_context.truncate(self.window_sizes.world);
        }
    }

    fn update_long_term_context(&mut self) {
        let mut seen = std::collections::HashSet::new();
        let mut long_term = Vec::new();

        for item in &self.context.episode_context {
            let key = format!("ep:{}", item.raw());
            if seen.insert(key.clone()) {
                long_term.push(key);
            }
        }

        for item in &self.context.semantic_context {
            let key = format!("sem:{}", item.raw());
            if seen.insert(key.clone()) {
                long_term.push(key);
            }
        }

        for item in &self.context.world_context {
            if seen.insert(item.clone()) {
                long_term.push(item.clone());
            }
        }

        if long_term.len() > self.window_sizes.long_term {
            long_term.truncate(self.window_sizes.long_term);
        }

        self.context.long_term_context = long_term;
    }

    fn compute_attention_weights(&mut self) {
        self.attention_weights.clear();

        let symbol_weight =
            self.context.symbol_context.len() as f32 / self.window_sizes.symbol.max(1) as f32;
        self.attention_weights
            .insert("symbol".into(), symbol_weight.min(1.0));

        let sentence_weight =
            self.context.sentence_context.len() as f32 / self.window_sizes.sentence.max(1) as f32;
        self.attention_weights
            .insert("sentence".into(), sentence_weight.min(1.0));

        let conversation_weight = self.context.conversation_context.len() as f32
            / self.window_sizes.conversation.max(1) as f32;
        self.attention_weights
            .insert("conversation".into(), conversation_weight.min(1.0));

        let episode_weight =
            self.context.episode_context.len() as f32 / self.window_sizes.episode.max(1) as f32;
        self.attention_weights
            .insert("episode".into(), episode_weight.min(1.0));

        let semantic_weight =
            self.context.semantic_context.len() as f32 / self.window_sizes.semantic.max(1) as f32;
        self.attention_weights
            .insert("semantic".into(), semantic_weight.min(1.0));

        let world_weight =
            self.context.world_context.len() as f32 / self.window_sizes.world.max(1) as f32;
        self.attention_weights
            .insert("world".into(), world_weight.min(1.0));

        let long_term_weight =
            self.context.long_term_context.len() as f32 / self.window_sizes.long_term.max(1) as f32;
        self.attention_weights
            .insert("long_term".into(), long_term_weight.min(1.0));
    }

    pub fn attention_for(&self, layer: &str) -> f32 {
        self.attention_weights.get(layer).copied().unwrap_or(0.0)
    }

    pub fn summary(&self) -> HashMap<String, usize> {
        let mut sizes = HashMap::new();
        sizes.insert("symbol".into(), self.context.symbol_context.len());
        sizes.insert("sentence".into(), self.context.sentence_context.len());
        sizes.insert(
            "conversation".into(),
            self.context.conversation_context.len(),
        );
        sizes.insert("episode".into(), self.context.episode_context.len());
        sizes.insert("semantic".into(), self.context.semantic_context.len());
        sizes.insert("world".into(), self.context.world_context.len());
        sizes.insert("long_term".into(), self.context.long_term_context.len());
        sizes
    }
}
