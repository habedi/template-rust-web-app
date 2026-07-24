use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::domain::error::{Error, Result};

// `u64` because `validator` length rules in the web layer expect that type.
pub const NAME_MAX_LEN: u64 = 128;
pub const DESCRIPTION_MAX_LEN: u64 = 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// The fields a caller supplies when creating or replacing an item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemDraft {
    name: String,
    description: Option<String>,
}

impl ItemDraft {
    /// Trims the input and rejects a name that is empty or too long. Keeping the
    /// invariant here means every layer above gets an item that is already valid.
    pub fn new(name: &str, description: Option<&str>) -> Result<Self> {
        let name = name.trim();
        if name.is_empty() {
            return Err(Error::validation("name must not be empty"));
        }
        if name.chars().count() as u64 > NAME_MAX_LEN {
            return Err(Error::validation(format!(
                "name must be at most {NAME_MAX_LEN} characters"
            )));
        }

        let description = match description.map(str::trim) {
            Some("") => None,
            Some(text) if text.chars().count() as u64 > DESCRIPTION_MAX_LEN => {
                return Err(Error::validation(format!(
                    "description must be at most {DESCRIPTION_MAX_LEN} characters"
                )));
            }
            Some(text) => Some(text.to_string()),
            None => None,
        };

        Ok(Self {
            name: name.to_string(),
            description,
        })
    }

    // Read accessors complete the type. Only tests read them today, since the
    // consuming methods below are what the application layer needs.
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[allow(dead_code)]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Builds a new item, stamping the identifier and timestamps.
    pub fn into_new_item(self, now: DateTime<Utc>) -> Item {
        Item {
            id: Uuid::new_v4(),
            name: self.name,
            description: self.description,
            created_at: now,
            updated_at: now,
        }
    }

    /// Applies the draft to an existing item, preserving its identifier and
    /// creation time.
    pub fn apply_to(self, item: Item, now: DateTime<Utc>) -> Item {
        Item {
            id: item.id,
            name: self.name,
            description: self.description,
            created_at: item.created_at,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_trims_name_and_description() {
        let draft = ItemDraft::new("  widget  ", Some("  a thing  ")).unwrap();
        assert_eq!(draft.name(), "widget");
        assert_eq!(draft.description(), Some("a thing"));
    }

    #[test]
    fn new_treats_a_blank_description_as_absent() {
        let draft = ItemDraft::new("widget", Some("   ")).unwrap();
        assert_eq!(draft.description(), None);
    }

    #[test]
    fn new_rejects_a_blank_name() {
        let err = ItemDraft::new("   ", None).unwrap_err();
        assert!(matches!(err, Error::Validation(_)));
    }

    #[test]
    fn new_rejects_an_overlong_name() {
        let err = ItemDraft::new(&"a".repeat(NAME_MAX_LEN as usize + 1), None).unwrap_err();
        assert!(matches!(err, Error::Validation(_)));
    }

    #[test]
    fn new_accepts_a_name_at_the_limit() {
        let name = "a".repeat(NAME_MAX_LEN as usize);
        assert_eq!(ItemDraft::new(&name, None).unwrap().name(), name);
    }

    #[test]
    fn new_rejects_an_overlong_description() {
        let description = "a".repeat(DESCRIPTION_MAX_LEN as usize + 1);
        let err = ItemDraft::new("widget", Some(&description)).unwrap_err();
        assert!(matches!(err, Error::Validation(_)));
    }

    #[test]
    fn apply_to_preserves_identity_and_creation_time() {
        let created_at = Utc::now();
        let item = ItemDraft::new("old", None)
            .unwrap()
            .into_new_item(created_at);
        let updated_at = created_at + chrono::Duration::seconds(1);

        let updated = ItemDraft::new("new", Some("text"))
            .unwrap()
            .apply_to(item.clone(), updated_at);

        assert_eq!(updated.id, item.id);
        assert_eq!(updated.created_at, created_at);
        assert_eq!(updated.updated_at, updated_at);
        assert_eq!(updated.name, "new");
    }
}
