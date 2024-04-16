use std::collections::BTreeMap;

use getset::Setters;
use serde::{Deserialize, Serialize};

use crate::{utils, WithID, WithXMLAttributes};

#[cfg(doc)]
use crate::Core;

/// Object representation of a [`Core`]'s router.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Setters)]
pub struct Router {
    /// The associated core id (not part of XML).
    #[serde(skip)]
    #[getset(set = "pub")]
    id: u8,
    /// Any other router attribute present in the XML.
    #[serde(
        flatten,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "utils::attrs::deserialize_attrs"
    )]
    other_attributes: Option<BTreeMap<String, String>>,
}

impl Router {
    #[cfg(test)]
    /// Instantiates a new [`Router`] instance.
    pub fn new(id: u8, other_attributes: Option<BTreeMap<String, String>>) -> Self {
        Self {
            id,
            other_attributes,
        }
    }
}

impl WithXMLAttributes for Router {
    fn other_attributes(&self) -> &Option<BTreeMap<String, String>> {
        &self.other_attributes
    }

    fn variant(&self) -> &'static str {
        &"r"
    }
}

impl WithID<u8> for Router {
    fn id(&self) -> &u8 {
        &self.id
    }
}
