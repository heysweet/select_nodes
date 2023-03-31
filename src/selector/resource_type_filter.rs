use crate::interface::{NodeType, self};
use interface::ResourceTypeFilter;

impl ResourceTypeFilter {
    pub fn should_include(&self, resource_type: NodeType) -> bool {
        match self {
            ResourceTypeFilter::All => true,
            ResourceTypeFilter::None => false,
            ResourceTypeFilter::Some(types) => types.contains(&resource_type),
        }
    }
}
