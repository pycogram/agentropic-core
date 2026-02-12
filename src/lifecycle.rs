/// Agent lifecycle states

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    /// Agent has been created but not initialized
    Created,
    /// Agent has been initialized
    Initialized,
    /// Agent is currently running
    Running,
    /// Agent execution is paused
    Paused,
    /// Agent has been stopped
    Stopped,
}

impl AgentState {
    pub fn can_transition_to(&self, next: AgentState) -> bool {
        matches!(
            (self, next),
            (AgentState::Created, AgentState::Initialized)
            | (AgentState::Initialized, AgentState::Running)
            | (AgentState::Initialized, AgentState::Stopped)
            | (AgentState::Running, AgentState::Paused)
            | (AgentState::Running, AgentState::Stopped)
            | (AgentState::Paused, AgentState::Running)
            | (AgentState::Paused, AgentState::Stopped)
        )
    }
}