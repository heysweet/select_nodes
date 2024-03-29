use crate::dbt_node_selector::{self, NodeType};
use crate::dbt_node_selector::ResourceTypeFilter;

impl ResourceTypeFilter {
    pub fn should_include(&self, resource_type: &NodeType) -> bool {
        match self {
            ResourceTypeFilter::All => true,
            ResourceTypeFilter::None => false,
            ResourceTypeFilter::Some(types) => types.iter().any(|t| t.key_matches(&resource_type)),
        }
    }
}
