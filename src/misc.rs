use Message;

/// TTL of a message.
///
/// It decreases by one each time the message is forwarded.
/// If the TTL of a message reaches zero,
/// the message will be handled by the node that keeps the message at the time.
/// So, a TTL can be regarded as the hop count of a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimeToLive(u8);
impl TimeToLive {
    /// Makes a new `TimeToLive` instance.
    pub fn new(ttl: u8) -> Self {
        TimeToLive(ttl)
    }

    /// Returns the value of the TTL.
    pub fn as_u8(self) -> u8 {
        self.0
    }

    /// Returns `true` if the TTL is expired, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyparview::TimeToLive;
    ///
    /// let ttl = TimeToLive::new(10);
    /// assert!(!ttl.is_expired());
    ///
    /// let ttl = TimeToLive::new(0);
    /// assert!(ttl.is_expired());
    /// ```
    pub fn is_expired(&self) -> bool {
        self.0 == 0
    }

    pub(crate) fn decrement(self) -> Self {
        TimeToLive(self.0.saturating_sub(1))
    }
}

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
        message: Message<T>,
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
    pub(crate) fn send(destination: T, message: Message<T>) -> Self {
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

/// Events emitted by HyParView [Node](./struct.Node.html).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event<T> {
    /// New neighbor node arrived.
    ///
    /// Internally, it means `node` was added to the active view of the local node.
    NeighborUp {
        /// The ID of the neighbor node.
        node: T,
    },

    /// A neighbor node departed.
    ///
    /// Internally, it means `node` was removed from the active view of the local node.
    NeighborDown {
        /// The ID of the neighbor node.
        node: T,
    },
}
