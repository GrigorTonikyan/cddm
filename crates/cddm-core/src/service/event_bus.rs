#![forbid(unsafe_code)]

use super::types::WorkspaceEvent;
use std::sync::Arc;
use tokio::sync::broadcast;

const DEFAULT_EVENT_CAPACITY: usize = 512;

/// Thread-safe reactive event bus for broadcasting domain events across all interfaces.
#[derive(Debug, Clone)]
pub struct EventBus {
    sender: Arc<broadcast::Sender<WorkspaceEvent>>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(DEFAULT_EVENT_CAPACITY)
    }
}

impl EventBus {
    /// Creates a new EventBus with the specified channel capacity buffer.
    pub fn new(capacity: usize) -> Self {
        let (sender, _receiver) = broadcast::channel(capacity);
        Self {
            sender: Arc::new(sender),
        }
    }

    /// Publishes a workspace domain event to all active subscribers.
    pub fn publish(&self, event: WorkspaceEvent) -> Result<usize, String> {
        // If there are no active receivers, send returns an error which is acceptable.
        match self.sender.send(event) {
            Ok(receiver_count) => Ok(receiver_count),
            Err(_) => Ok(0),
        }
    }

    /// Subscribes to the stream of reactive workspace events.
    pub fn subscribe(&self) -> broadcast::Receiver<WorkspaceEvent> {
        self.sender.subscribe()
    }

    /// Returns the current number of active event subscribers.
    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}
