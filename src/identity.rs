use crate::AgentId;

/// Agent identity with ID and name
#[derive(Debug, Clone)]
pub struct AgentIdentity {
    id: AgentId,
    name: String,
}

impl AgentIdentity {
    pub fn new(id: AgentId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }
    
    pub fn id(&self) -> &AgentId {
        &self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
}