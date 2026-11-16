// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-ceph: ceph component model.
//!
//! Real implementation: ceph_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CephItem {
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
pub enum CephError {
    EmptyCephItemName,
    DuplicateCephItem(String),
    EmptyEventSubject,
}

impl fmt::Display for CephError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCephItemName => write!(f, "ceph_item name must not be empty"),
            Self::DuplicateCephItem(id) => write!(f, "duplicate ceph_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for CephError {}

pub struct Ceph {
    pub name: String,
    ceph_items: BTreeMap<String, CephItem>,
    events: BTreeMap<String, Event>,
}

impl Ceph {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ceph_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_ceph_item(&mut self, item: CephItem) -> Result<(), CephError> {
        if item.name.trim().is_empty() {
            return Err(CephError::EmptyCephItemName);
        }
        let id = item.id.clone();
        if self.ceph_items.contains_key(&id) {
            return Err(CephError::DuplicateCephItem(id.clone()));
        }
        self.ceph_items.insert(id, item);
        Ok(())
    }

    pub fn ceph_item_count(&self) -> usize {
        self.ceph_items.len()
    }

    pub fn get_ceph_item(&self, id: &str) -> Option<&CephItem> {
        self.ceph_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), CephError> {
        if event.subject.trim().is_empty() {
            return Err(CephError::EmptyEventSubject);
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

    fn sample_ceph_item(id: &str, name: &str) -> CephItem {
        CephItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Ceph::new("Ceph");
        assert_eq!(m.name, "Ceph");
        assert_eq!(m.ceph_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_ceph_item() {
        let mut m = Ceph::new("Ceph");
        m.add_ceph_item(sample_ceph_item("a1", "Alpha")).unwrap();
        assert_eq!(m.ceph_item_count(), 1);
        assert!(m.get_ceph_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Ceph::new("Ceph");
        assert!(m.add_ceph_item(sample_ceph_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Ceph::new("Ceph");
        m.add_ceph_item(sample_ceph_item("x", "One")).unwrap();
        assert!(m.add_ceph_item(sample_ceph_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Ceph::new("Ceph");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
