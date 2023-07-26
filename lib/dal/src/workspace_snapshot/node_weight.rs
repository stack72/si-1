//! Nodes

pub use crate::workspace_snapshot::node_weight::content_node_weight::ContentKind;
use crate::workspace_snapshot::{
    change_set::{ChangeSet, ChangeSetError},
    content_hash::ContentHash,
    vector_clock::{VectorClock, VectorClockError},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod content_node_weight;
pub mod ordering_node_weight;

pub use content_node_weight::ContentNodeWeight;
pub use ordering_node_weight::OrderingNodeWeight;
use ulid::Ulid;

#[derive(Debug, Error)]
pub enum NodeWeightError {
    #[error("Cannot set content hash directly on node weight kind")]
    CannotSetContentHashOnKind,
    #[error("Cannot update root node's content hash")]
    CannotUpdateRootNodeContentHash,
    #[error("ChangeSet error: {0}")]
    ChangeSet(#[from] ChangeSetError),
    #[error("Incompatible node weights")]
    IncompatibleNodeWeightVariants,
    #[error("No Seen Vector Clock available")]
    NoSeenVectorClock,
    #[error("Vector Clock error: {0}")]
    VectorClock(#[from] VectorClockError),
}

pub type NodeWeightResult<T> = Result<T, NodeWeightError>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NodeWeight {
    Content(ContentNodeWeight),
    Ordering(OrderingNodeWeight),
}

impl NodeWeight {
    pub fn content_hash(&self) -> ContentHash {
        match self {
            NodeWeight::Content(content_weight) => content_weight.content_hash(),
            NodeWeight::Ordering(ordering_weight) => ordering_weight.content_hash(),
        }
    }

    pub fn id(&self) -> Ulid {
        match self {
            NodeWeight::Content(content_weight) => content_weight.id(),
            NodeWeight::Ordering(ordering_weight) => ordering_weight.id(),
        }
    }

    pub fn increment_seen_vector_clock(&mut self, change_set: &ChangeSet) -> NodeWeightResult<()> {
        match self {
            NodeWeight::Content(content_weight) => {
                content_weight.increment_seen_vector_clock(change_set)
            }
            NodeWeight::Ordering(ordering_weight) => {
                ordering_weight.increment_seen_vector_clock(change_set)
            }
        }
    }

    pub fn merge_clocks(
        &mut self,
        change_set: &ChangeSet,
        other: &NodeWeight,
    ) -> NodeWeightResult<()> {
        match (self, other) {
            (
                NodeWeight::Content(self_content_weight),
                NodeWeight::Content(other_content_weight),
            ) => self_content_weight.merge_clocks(change_set, other_content_weight),
            (
                NodeWeight::Ordering(self_ordering_weight),
                NodeWeight::Ordering(other_ordering_weight),
            ) => self_ordering_weight.merge_clocks(change_set, other_ordering_weight),
            _ => Err(NodeWeightError::IncompatibleNodeWeightVariants),
        }
    }

    pub fn merkle_tree_hash(&self) -> ContentHash {
        match self {
            NodeWeight::Content(content_weight) => content_weight.merkle_tree_hash(),
            NodeWeight::Ordering(ordering_weight) => ordering_weight.merkle_tree_hash(),
        }
    }

    pub fn new_content_hash(&mut self, content_hash: ContentHash) -> NodeWeightResult<()> {
        match self {
            NodeWeight::Content(content_weight) => content_weight.new_content_hash(content_hash),
            NodeWeight::Ordering(_) => Err(NodeWeightError::CannotSetContentHashOnKind),
        }
    }

    pub fn new_content(
        change_set: &ChangeSet,
        content_id: Ulid,
        kind: ContentKind,
    ) -> NodeWeightResult<Self> {
        Ok(NodeWeight::Content(ContentNodeWeight::new(
            change_set, content_id, kind,
        )?))
    }

    pub fn new_content_with_seen_vector_clock(
        change_set: &ChangeSet,
        kind: ContentKind,
    ) -> NodeWeightResult<Self> {
        Ok(NodeWeight::Content(
            ContentNodeWeight::new_with_seen_vector_clock(change_set, kind)?,
        ))
    }

    pub fn new_with_incremented_vector_clocks(
        &self,
        change_set: &ChangeSet,
    ) -> NodeWeightResult<Self> {
        let new_weight = match self {
            NodeWeight::Content(content_weight) => {
                NodeWeight::Content(content_weight.new_with_incremented_vector_clocks(change_set)?)
            }
            NodeWeight::Ordering(ordering_weight) => NodeWeight::Ordering(
                ordering_weight.new_with_incremented_vector_clocks(change_set)?,
            ),
        };

        Ok(new_weight)
    }

    pub fn set_merkle_tree_hash(&mut self, new_hash: ContentHash) {
        match self {
            NodeWeight::Content(content_weight) => content_weight.set_merkle_tree_hash(new_hash),
            NodeWeight::Ordering(ordering_weight) => ordering_weight.set_merkle_tree_hash(new_hash),
        }
    }

    pub fn vector_clock_seen(&self) -> Option<&VectorClock> {
        match self {
            NodeWeight::Content(content_weight) => content_weight.vector_clock_seen(),
            NodeWeight::Ordering(ordering_weight) => Some(ordering_weight.vector_clock_seen()),
        }
    }

    pub fn vector_clock_write(&self) -> &VectorClock {
        match self {
            NodeWeight::Content(content_weight) => content_weight.vector_clock_write(),
            NodeWeight::Ordering(ordering_weight) => ordering_weight.vector_clock_write(),
        }
    }
}
