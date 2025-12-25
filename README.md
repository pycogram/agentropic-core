# Agentropic Core

Agentropic Core defines the **foundational programming paradigm** for Agent-Oriented Programming (AOP).

It answers one question:

> **What is an agent, formally and programmatically?**

It provides the core abstractions, lifecycles, and communication models that all Agentropic-based systems rely on.

---

## 🎯 Scope

- **Agent** – Autonomous entity with beliefs, goals, and plans
- **Beliefs** – Agent’s internal representation of the world
- **Goals** – Desired future states
- **Plans** – Ordered actions to achieve goals
- **Actions** – Atomic operations
- **Decision Engine** – Strategy for choosing plans

---

## 🧱 Architecture

```text
agentropic-core/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   │
│   ├── agent/
│   │   ├── mod.rs
│   │   ├── agent.rs
│   │   ├── lifecycle.rs
│   │   └── context.rs
│   │
│   ├── belief/
│   │   ├── mod.rs
│   │   └── belief.rs
│   │
│   ├── goal/
│   │   ├── mod.rs
│   │   └── goal.rs
│   │
│   ├── plan/
│   │   ├── mod.rs
│   │   └── plan.rs
│   │
│   ├── action/
│   │   ├── mod.rs
│   │   └── action.rs
│   │
│   ├── decision/
│   │   ├── mod.rs
│   │   └── decision.rs
│   │
│   ├── message/
│   │   ├── mod.rs
│   │   └── message.rs
│   │
│   ├── error.rs
│   └── prelude.rs
│
└── tests/
    └── basic_agent.rs
```

---

## 🧠 Design Principles

- Agents are **first-class program entities**
- Behavior is explicit and inspectable
- No hard dependency on AI models
- Rust traits define agent capabilities

---

## 🚫 Out of Scope

- LLM integrations
- Execution engines
- Deployment concerns

---

## 🔗 Part of Agentropic

Agentropic Core is used by:

- **agentropic-runtime**: Agent execution and scheduling
- **agentropic-systems**: Multi-agent system patterns
- **agentropic-examples**: Reference implementations

---

## 🛠️ Usage

```text
use agentropic_core::prelude::*;

let mut agent = Agent::new();
agent.goals.push(Goal {
    description: "Explore environment".into(),
    achieved: false,
});
```

---

