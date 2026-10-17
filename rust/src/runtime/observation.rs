use crate::{ObservatoryEvent, ObservatorySnapshot};
use chrono::{DateTime, Utc};
use serde_json::{Value, json};
use std::collections::VecDeque;

pub struct Observation {
    pub provider_id: String,
    pub provider_name: String,
    pub source: String,
    pub thread_id: String,
    pub title: String,
    pub revision: u64,
    pub epoch: String,
    pub public_summary: Option<String>,
    pub summary: String,
    pub state: &'static str,
    pub observed_at: String,
    pub events: VecDeque<ObservatoryEvent>,
    pub error: Option<String>,
}
impl Observation {
    pub fn at(
        provider_id: &str,
        provider_name: &str,
        source: &str,
        thread: &Value,
        now: DateTime<Utc>,
        epoch: String,
    ) -> Self {
        Self {
            provider_id: provider_id.into(),
            provider_name: provider_name.into(),
            source: source.into(),
            thread_id: thread["id"].as_str().unwrap_or_default().into(),
            title: thread["name"].as_str().unwrap_or("Agent session").into(),
            revision: 0,
            epoch,
            public_summary: None,
            summary: "Waiting for public session activity".into(),
            state: "waiting",
            observed_at: now.to_rfc3339(),
            events: VecDeque::new(),
            error: None,
        }
    }
    pub fn activity(&mut self, timestamp: &str, summary: String, state: &'static str) {
        self.revision += 1;
        self.summary = summary;
        self.state = state;
        self.observed_at = timestamp.into();
        let event = json!({"protocolVersion":"0.1","eventId":format!("{}:{}:{}",self.thread_id,self.epoch,self.revision),"cursor":self.cursor(self.revision),
            "providerId":self.provider_id,"occurredAt":timestamp,"kind":"activity","subject":{"kind":"task","id":self.thread_id},
            "summary":self.summary,"payload":{"revision":self.revision.to_string()},"extensions":null});
        if let Ok(event) = serde_json::from_value(event) {
            self.events.push_back(event);
        }
        while self.events.len() > 256 {
            self.events.pop_front();
        }
    }
    pub fn snapshot(&self) -> Result<ObservatorySnapshot, serde_json::Error> {
        serde_json::from_value(
            json!({"protocolVersion":"0.1","snapshotId":format!("{}:{}:{}",self.provider_id,self.epoch,self.revision),"cursor":self.cursor(self.revision),"observedAt":self.observed_at,
            "providers":[self.descriptor()],"workItems":[{"id":self.thread_id,"providerId":self.provider_id,"kind":"task","title":self.title,"summary":self.public_summary.as_ref().unwrap_or(&self.summary),
            "lifecycle":self.state,"phase":"unknown","attention":"none","health":if self.error.is_some(){"unreachable"}else{"healthy"},"providerState":if self.state=="active"{"activity_observed"}else{"idle_observed"},
            "revision":self.revision.to_string(),"dependencyIds":[],"actions":[],"updatedAt":self.observed_at,
            "extensions":{"dev.miilo.session":{"source":self.source,"timestampOrigin":if self.source=="cursor_transcript"{"observed"}else{"source"},"readOnly":true,"runtimeStatusAuthoritative":false}}}],
            "actors":[],"attempts":[],"artifacts":[],"evidence":[],"attention":[]}),
        )
    }
    pub fn cursor(&self, revision: u64) -> String {
        format!("{}:{revision}", self.epoch)
    }
    pub fn events_after(
        &self,
        cursor: Option<&str>,
    ) -> Result<Vec<ObservatoryEvent>, &'static str> {
        let after = match cursor {
            Some(c) => {
                let (epoch, seq) = c.split_once(':').ok_or("invalid cursor")?;
                if epoch != self.epoch {
                    return Err("observer restarted; fetch snapshot");
                }
                seq.parse::<u64>().map_err(|_| "invalid cursor")?
            }
            None => self.revision.saturating_sub(1),
        };
        if after > self.revision
            || self.events.front().is_some_and(|e| {
                after.saturating_add(1)
                    < e.cursor
                        .rsplit(':')
                        .next()
                        .and_then(|v| v.parse::<u64>().ok())
                        .unwrap_or(0)
            })
        {
            return Err("cursor expired; fetch snapshot");
        }
        Ok(self
            .events
            .iter()
            .filter(|e| {
                e.cursor
                    .rsplit(':')
                    .next()
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(0)
                    > after
            })
            .cloned()
            .collect())
    }
    pub fn descriptor(&self) -> Value {
        json!({"id":self.provider_id,"name":self.provider_name,"kind":"agent_harness","health":if self.error.is_some(){"unreachable"}else{"healthy"},
            "capabilities":{"observe":["tasks"],"control":[],"features":{"readOnly":true,"streaming":false}},
            "extensions":{"dev.miilo.bridge":{"protocolVersion":"0.1","version":env!("CARGO_PKG_VERSION")}}})
    }
}
