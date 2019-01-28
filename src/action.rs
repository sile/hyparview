use crate::message::ProtocolMessage;
use crate::Event;

/// Actions instructed by HyParView [Node](./struct.Node.html).
///
/// For running HyParView nodes, the users must handle the actions correctly.
#[derive(Debug, PartialEq, Eq)]
pub enum Action<T> {
    /// Send a message.
    ///
    /// If there is no existing connection between the sender and the destination,
    /// new connection should be established automatically.
    ///
    /// If the destination node does not exist, the message will be discarded silently.
    ///
    /// Although it is not recommended,
    /// it is acceptable for discarding some messages
    /// if the load of networks (or systems) are too high.
    Send {
        /// The ID of the destination node of the message.
        destination: T,

        /// An outgoing message.
        message: ProtocolMessage<T>,
    },

    /// Close a connection.
    ///
    /// The connection between the local node and `node` must be disconnected.
    /// If there is no such connection, this action will be silently ignored.
    Disconnect {
        /// The ID of the target node.
        node: T,
    },

    /// Notify an event.
    ///
    /// If there are some listeners that monitoring HyPerView events,
    /// the events emitted by nodes should be notified to them.
    Notify {
        /// An event emitted by a HyParView [Node](./struct.Node.html).
        event: Event<T>,
    },
}
impl<T> Action<T> {
    pub(crate) fn send(destination: T, message: ProtocolMessage<T>) -> Self {
        Action::Send {
            destination,
            message,
        }
    }

    pub(crate) fn disconnect(node: T) -> Self {
        Action::Disconnect { node }
    }

    pub(crate) fn notify_up(node: T) -> Self {
        Action::Notify {
            event: Event::NeighborUp { node },
        }
    }

    pub(crate) fn notify_down(node: T) -> Self {
        Action::Notify {
            event: Event::NeighborDown { node },
        }
    }
}
