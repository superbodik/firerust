use std::collections::HashMap;
use std::sync::Arc;

use fr_dom::node::node_id::NodeId;

use crate::decode::decoded_image::DecodedImage;

#[derive(Debug, Clone, Default)]
pub struct ImageStore {
    images: HashMap<NodeId, Arc<DecodedImage>>,
}

impl ImageStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, element: NodeId, image: Arc<DecodedImage>) {
        self.images.insert(element, image);
    }

    pub fn get(&self, element: NodeId) -> Option<&Arc<DecodedImage>> {
        self.images.get(&element)
    }

    pub fn len(&self) -> usize {
        self.images.len()
    }

    pub fn is_empty(&self) -> bool {
        self.images.is_empty()
    }
}
