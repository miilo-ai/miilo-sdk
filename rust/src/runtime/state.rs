use crate::{
    EventKind, MIILO_PROTOCOL_VERSION, ObservatoryEvent, ObservatorySnapshot, ProviderDescriptor,
    SubjectKind, SubjectReference,
};
use chrono::{DateTime, Utc};
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    InvalidSnapshot(String),
    AlreadyRegistered(String),
    NotRegistered(String),
    CursorExpired,
    ProviderBoundary(String),
}
impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSnapshot(message) => write!(f, "invalid snapshot: {message}"),
            Self::AlreadyRegistered(id) => write!(f, "provider {id} already registered"),
            Self::NotRegistered(id) => write!(f, "provider {id} is not registered"),
            Self::ProviderBoundary(id) => {
                write!(f, "snapshot contains records outside provider {id}")
            }
            Self::CursorExpired => write!(f, "cursor unavailable; reconcile with a snapshot"),
        }
    }
}
impl std::error::Error for RuntimeError {}

pub struct ObservatoryRuntime {
    epoch: String,
    sequence: u64,
    providers: BTreeMap<String, ProviderDescriptor>,
    snapshots: BTreeMap<String, ObservatorySnapshot>,
    history: VecDeque<ObservatoryEvent>,
    capacity: usize,
}
impl ObservatoryRuntime {
    pub fn new(epoch: impl Into<String>, capacity: usize) -> Result<Self, RuntimeError> {
        let epoch = epoch.into();
        if epoch.is_empty()
            || epoch.len() > 96
            || !epoch
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "-_".contains(c))
            || !(1..=4096).contains(&capacity)
        {
            return Err(RuntimeError::InvalidSnapshot(
                "invalid runtime epoch/capacity".into(),
            ));
        }
        Ok(Self {
            epoch,
            sequence: 0,
            providers: BTreeMap::new(),
            snapshots: BTreeMap::new(),
            history: VecDeque::new(),
            capacity,
        })
    }
    pub fn register(
        &mut self,
        provider: ProviderDescriptor,
        now: DateTime<Utc>,
    ) -> Result<ObservatoryEvent, RuntimeError> {
        if provider.id.trim().is_empty() || provider.name.trim().is_empty() {
            return Err(RuntimeError::InvalidSnapshot(
                "provider ID/name required".into(),
            ));
        }
        if self.providers.contains_key(&provider.id) {
            return Err(RuntimeError::AlreadyRegistered(provider.id));
        }
        if self.providers.len() >= 1000 {
            return Err(RuntimeError::InvalidSnapshot(
                "provider capacity exceeded".into(),
            ));
        }
        let id = provider.id.clone();
        self.providers.insert(id.clone(), provider);
        Ok(self.record(
            &id,
            EventKind::Created,
            "Provider connection registered.",
            now,
        ))
    }
    pub fn replace(
        &mut self,
        id: &str,
        snapshot: ObservatorySnapshot,
        now: DateTime<Utc>,
    ) -> Result<Option<ObservatoryEvent>, RuntimeError> {
        super::validate_snapshot(id, &snapshot)?;
        if !self.providers.contains_key(id) {
            return Err(RuntimeError::NotRegistered(id.into()));
        }
        if self.snapshots.get(id).is_some_and(|old| {
            serde_json::to_value(old).ok() == serde_json::to_value(&snapshot).ok()
        }) {
            return Ok(None);
        }
        self.snapshots.insert(id.into(), snapshot);
        Ok(Some(self.record(
            id,
            EventKind::Updated,
            "Provider snapshot ingested.",
            now,
        )))
    }
    pub fn disconnect(
        &mut self,
        id: &str,
        now: DateTime<Utc>,
    ) -> Result<ObservatoryEvent, RuntimeError> {
        if self.providers.remove(id).is_none() {
            return Err(RuntimeError::NotRegistered(id.into()));
        }
        self.snapshots.remove(id);
        Ok(self.record(
            id,
            EventKind::Deleted,
            "Provider connection disconnected.",
            now,
        ))
    }
    pub fn snapshot(&self, now: DateTime<Utc>) -> ObservatorySnapshot {
        let mut out = ObservatorySnapshot {
            protocol_version: MIILO_PROTOCOL_VERSION.into(),
            snapshot_id: format!("runtime-{}-{}", self.epoch, self.sequence),
            cursor: self.cursor(),
            observed_at: now,
            providers: vec![],
            work_items: vec![],
            actors: vec![],
            attempts: vec![],
            artifacts: vec![],
            evidence: vec![],
            attention: vec![],
            extensions: None,
        };
        for (id, descriptor) in &self.providers {
            if let Some(value) = self.snapshots.get(id) {
                out.providers.extend(value.providers.clone());
                out.work_items.extend(value.work_items.clone());
                out.actors.extend(value.actors.clone());
                out.attempts.extend(value.attempts.clone());
                out.artifacts.extend(value.artifacts.clone());
                out.evidence.extend(value.evidence.clone());
                out.attention.extend(value.attention.clone());
            } else {
                out.providers.push(descriptor.clone());
            }
        }
        out
    }
    pub fn cursor(&self) -> String {
        format!("{}:{}", self.epoch, self.sequence)
    }
    pub fn events_after(
        &self,
        cursor: Option<&str>,
    ) -> Result<Vec<ObservatoryEvent>, RuntimeError> {
        let Some(cursor) = cursor else {
            return Ok(vec![]);
        };
        let (epoch, number) = cursor.rsplit_once(':').ok_or(RuntimeError::CursorExpired)?;
        let number: u64 = number.parse().map_err(|_| RuntimeError::CursorExpired)?;
        let oldest = self.sequence.saturating_sub(self.history.len() as u64);
        if epoch != self.epoch || number > self.sequence || number < oldest {
            return Err(RuntimeError::CursorExpired);
        }
        Ok(self
            .history
            .iter()
            .skip((number - oldest) as usize)
            .cloned()
            .collect())
    }
    fn record(
        &mut self,
        id: &str,
        kind: EventKind,
        summary: &str,
        now: DateTime<Utc>,
    ) -> ObservatoryEvent {
        self.sequence += 1;
        let event = ObservatoryEvent {
            protocol_version: MIILO_PROTOCOL_VERSION.into(),
            event_id: self.cursor(),
            cursor: self.cursor(),
            provider_id: id.into(),
            occurred_at: now,
            kind,
            subject: SubjectReference {
                kind: SubjectKind::Provider,
                id: id.into(),
            },
            summary: summary.into(),
            payload: Default::default(),
            extensions: None,
        };
        self.history.push_back(event.clone());
        while self.history.len() > self.capacity {
            self.history.pop_front();
        }
        event
    }
}
