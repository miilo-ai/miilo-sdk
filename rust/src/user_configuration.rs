//! User configuration contract. Filesystem access and migration belong to clients.
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserConfiguration {
    pub schema_version: u32,
    #[serde(
        default,
        deserialize_with = "optional_path",
        skip_serializing_if = "Option::is_none"
    )]
    pub local_config_file: Option<String>,
    /// Preserve additive settings when a client rewrites the manifest.
    #[serde(flatten)]
    pub extensions: BTreeMap<String, Value>,
}

fn optional_path<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
    // An omitted path is supported; explicit null is not a path in the schema.
    String::deserialize(deserializer).map(Some)
}

impl UserConfiguration {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.schema_version != 1
            || self
                .local_config_file
                .as_ref()
                .is_some_and(|path| !path.starts_with('/'))
        {
            return Err(
                "Unsupported Miilo configuration; expected schemaVersion 1 and an absolute localConfigFile",
            );
        }
        Ok(())
    }
}
