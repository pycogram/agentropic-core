/// Agent context for runtime information

use std::sync::Arc;
use crate::AgentId;

#[derive(Debug, Clone)]
pub struct AgentContext {
    agent_id: AgentId,
    config: Arc<dyn std::any::Any + Send + Sync>,
}

impl AgentContext {
    pub fn new(agent_id: AgentId) -> Self {
        Self {
            agent_id,
            config: Arc::new(()),
        }
    }
    
    pub fn agent_id(&self) -> &AgentId {
        &self.agent_id
    }
    
    pub fn log_info(&self, msg: &str) {
        println!("[INFO] [{}] {}", self.agent_id, msg);
    }
    
    pub fn log_error(&self, msg: &str) {
        eprintln!("[ERROR] [{}] {}", self.agent_id, msg);
    }
}