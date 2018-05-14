use TimeToLive;

/// Messages used for inter-node communication.
///
/// Unlike the [paper][HyParView], this does not contain `DISCONNECT` message.
/// In this crate, disconnections are assumed to be handled at the out of the normal messages
/// (e.g., Disconnections at the TCP level may be used for that purpose).
///
/// [HyParView]: http://asc.di.fct.unl.pt/~jleitao/pdf/dsn07-leitao.pdf
#[derive(Debug, Clone)]
pub enum Message<T> {
    /// `JOIN` message.
    ///
    /// This is sent by new nodes for joining a HyParView cluster.
    /// The receiver is the contact node of the cluster.
    Join {
        /// The node ID of the message sender.
        ///
        /// It is also a new node that wishes to join the cluster.
        sender: T,
    },

    /// `FORWARD_JOIN` message.
    ///
    /// This is used for disseminating a `JOIN` request to the members of the cluster to
    /// which the contact node belongs.
    ForwardJoin {
        /// The node ID of the message sender.
        sender: T,

        /// The ID of the new joining node.
        new_node: T,

        /// TTL of the message.
        ttl: TimeToLive,
    },

    /// `NEIGHBOR` message.
    ///
    /// This is used for refilling active view shrunk by node disconnections.
    ///
    /// In this crate, it is also used for notifing to new node that
    /// HyParView level connection has been established
    /// (in that case the value of `high_priority` always be set to `true`).
    Neighbor {
        /// The node ID of the message sender.
        sender: T,

        /// Whether the priority of the sender is high or low.
        high_priority: bool,
    },

    /// `SHUFFLE` message.
    ///
    /// This and `SHUFFLE_REPLY` messages are used for shuffling passive views of two nodes.
    Shuffle {
        /// The node ID of the message sender.
        sender: T,

        /// The ID of the origin node that emitted the shuffle request.
        origin: T,

        /// The nodes selected by `origin` for shuffling.
        nodes: Vec<T>,

        /// TTL of the message.
        ttl: TimeToLive,
    },

    /// `SHUFFLE_REPLY` message.
    ShuffleReply {
        /// The node ID of the message sender.
        sender: T,

        /// The nodes selected by `sender` as the reply of the associated `Shuffle` message.
        nodes: Vec<T>,
    },
}
impl<T> Message<T> {
    /// Returns the node ID of the sender of the message.
    pub fn sender(&self) -> &T {
        match self {
            Message::Join { sender } => sender,
            Message::ForwardJoin { sender, .. } => sender,
            Message::Neighbor { sender, .. } => sender,
            Message::Shuffle { sender, .. } => sender,
            Message::ShuffleReply { sender, .. } => sender,
        }
    }
}
impl<T: Clone> Message<T> {
    pub(crate) fn join(sender: &T) -> Self {
        Message::Join {
            sender: sender.clone(),
        }
    }

    pub(crate) fn forward_join(sender: &T, new_node: T, ttl: TimeToLive) -> Self {
        Message::ForwardJoin {
            sender: sender.clone(),
            new_node,
            ttl,
        }
    }

    pub(crate) fn neighbor(sender: &T, high_priority: bool) -> Self {
        Message::Neighbor {
            sender: sender.clone(),
            high_priority,
        }
    }

    pub(crate) fn shuffle(sender: &T, origin: T, nodes: Vec<T>, ttl: TimeToLive) -> Self {
        Message::Shuffle {
            sender: sender.clone(),
            origin,
            nodes,
            ttl,
        }
    }

    pub(crate) fn shuffle_reply(sender: &T, nodes: Vec<T>) -> Self {
        Message::ShuffleReply {
            sender: sender.clone(),
            nodes,
        }
    }
}
