// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-module-deps: module deps component model.
//!
//! Real implementation: depss, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deps {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleDepsError {
    EmptyDepsName,
    DuplicateDeps(String),
    EmptyEventSubject,
}

impl fmt::Display for ModuleDepsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDepsName => write!(f, "deps name must not be empty"),
            Self::DuplicateDeps(id) => write!(f, "duplicate deps {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ModuleDepsError {}

pub struct ModuleDeps {
    pub name: String,
    depss: BTreeMap<String, Deps>,
    events: BTreeMap<String, Event>,
}

impl ModuleDeps {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            depss: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_deps(&mut self, item: Deps) -> Result<(), ModuleDepsError> {
        if item.name.trim().is_empty() {
            return Err(ModuleDepsError::EmptyDepsName);
        }
        let id = item.id.clone();
        if self.depss.contains_key(&id) {
            return Err(ModuleDepsError::DuplicateDeps(id.clone()));
        }
        self.depss.insert(id, item);
        Ok(())
    }

    pub fn deps_count(&self) -> usize {
        self.depss.len()
    }

    pub fn get_deps(&self, id: &str) -> Option<&Deps> {
        self.depss.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ModuleDepsError> {
        if event.subject.trim().is_empty() {
            return Err(ModuleDepsError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_deps(id: &str, name: &str) -> Deps {
        Deps {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ModuleDeps::new("ModuleDeps");
        assert_eq!(m.name, "ModuleDeps");
        assert_eq!(m.deps_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_deps() {
        let mut m = ModuleDeps::new("ModuleDeps");
        m.add_deps(sample_deps("a1", "Alpha")).unwrap();
        assert_eq!(m.deps_count(), 1);
        assert!(m.get_deps("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ModuleDeps::new("ModuleDeps");
        assert!(m.add_deps(sample_deps("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ModuleDeps::new("ModuleDeps");
        m.add_deps(sample_deps("x", "One")).unwrap();
        assert!(m.add_deps(sample_deps("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ModuleDeps::new("ModuleDeps");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
