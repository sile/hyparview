//! Messages used for inter-node communication.
use TimeToLive;

/// Messages used for inter-node communication.
///
/// [HyParView]: http://asc.di.fct.unl.pt/~jleitao/pdf/dsn07-leitao.pdf
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolMessage<T> {
    /// `JOIN` message.
    Join(JoinMessage<T>),

    /// `FORWARD_JOIN` message.
    ForwardJoin(ForwardJoinMessage<T>),

    /// `NEIGHBOR` message.
    Neighbor(NeighborMessage<T>),

    /// `SHUFFLE` message.
    Shuffle(ShuffleMessage<T>),

    /// `SHUFFLE_REPLY` message.
    ShuffleReply(ShuffleReplyMessage<T>),

    /// `DISCONNECT` messsage.
    Disconnect(DisconnectMessage<T>),
}
impl<T> ProtocolMessage<T> {
    /// Returns the node ID of the sender of the message.
    pub fn sender(&self) -> &T {
        match self {
            ProtocolMessage::Join(m) => &m.sender,
            ProtocolMessage::ForwardJoin(m) => &m.sender,
            ProtocolMessage::Neighbor(m) => &m.sender,
            ProtocolMessage::Shuffle(m) => &m.sender,
            ProtocolMessage::ShuffleReply(m) => &m.sender,
            ProtocolMessage::Disconnect(m) => &m.sender,
        }
    }
}
impl<T: Clone> ProtocolMessage<T> {
    pub(crate) fn join(sender: &T) -> Self {
        ProtocolMessage::Join(JoinMessage {
            sender: sender.clone(),
        })
    }

    pub(crate) fn forward_join(sender: &T, new_node: T, ttl: TimeToLive) -> Self {
        ProtocolMessage::ForwardJoin(ForwardJoinMessage {
            sender: sender.clone(),
            new_node,
            ttl,
        })
    }

    pub(crate) fn neighbor(sender: &T, high_priority: bool) -> Self {
        ProtocolMessage::Neighbor(NeighborMessage {
            sender: sender.clone(),
            high_priority,
        })
    }

    pub(crate) fn shuffle(sender: &T, origin: T, nodes: Vec<T>, ttl: TimeToLive) -> Self {
        ProtocolMessage::Shuffle(ShuffleMessage {
            sender: sender.clone(),
            origin,
            nodes,
            ttl,
        })
    }

    pub(crate) fn shuffle_reply(sender: &T, nodes: Vec<T>) -> Self {
        ProtocolMessage::ShuffleReply(ShuffleReplyMessage {
            sender: sender.clone(),
            nodes,
        })
    }

    pub(crate) fn disconnect(sender: &T, alive: bool) -> Self {
        ProtocolMessage::Disconnect(DisconnectMessage {
            sender: sender.clone(),
            alive,
        })
    }
}
impl<T> From<JoinMessage<T>> for ProtocolMessage<T> {
    fn from(f: JoinMessage<T>) -> Self {
        ProtocolMessage::Join(f)
    }
}
impl<T> From<ForwardJoinMessage<T>> for ProtocolMessage<T> {
    fn from(f: ForwardJoinMessage<T>) -> Self {
        ProtocolMessage::ForwardJoin(f)
    }
}
impl<T> From<NeighborMessage<T>> for ProtocolMessage<T> {
    fn from(f: NeighborMessage<T>) -> Self {
        ProtocolMessage::Neighbor(f)
    }
}
impl<T> From<ShuffleMessage<T>> for ProtocolMessage<T> {
    fn from(f: ShuffleMessage<T>) -> Self {
        ProtocolMessage::Shuffle(f)
    }
}
impl<T> From<ShuffleReplyMessage<T>> for ProtocolMessage<T> {
    fn from(f: ShuffleReplyMessage<T>) -> Self {
        ProtocolMessage::ShuffleReply(f)
    }
}
impl<T> From<DisconnectMessage<T>> for ProtocolMessage<T> {
    fn from(f: DisconnectMessage<T>) -> Self {
        ProtocolMessage::Disconnect(f)
    }
}

/// `JOIN` message.
///
/// This is sent by new nodes for joining a HyParView cluster.
/// The receiver is the contact node of the cluster.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinMessage<T> {
    /// The node ID of the message sender.
    ///
    /// It is also a new node that wishes to join the cluster.
    pub sender: T,
}

/// `FORWARD_JOIN` message.
///
/// This is used for disseminating a `JOIN` request to the members of the cluster to
/// which the contact node belongs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardJoinMessage<T> {
    /// The node ID of the message sender.
    pub sender: T,

    /// The ID of the new joining node.
    pub new_node: T,

    /// TTL of the message.
    pub ttl: TimeToLive,
}

/// `NEIGHBOR` message.
///
/// This is used for refilling active view shrunk by node disconnections.
///
/// In this crate, it is also used for notifing to new node that
/// HyParView level connection has been established
/// (in that case the value of `high_priority` always be set to `true`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeighborMessage<T> {
    /// The node ID of the message sender.
    pub sender: T,

    /// Whether the priority of the sender is high or low.
    pub high_priority: bool,
}

/// `SHUFFLE` message.
///
/// This and `SHUFFLE_REPLY` messages are used for shuffling passive views of two nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShuffleMessage<T> {
    /// The node ID of the message sender.
    pub sender: T,

    /// The ID of the origin node that emitted the shuffle request.
    pub origin: T,

    /// The nodes selected by `origin` for shuffling.
    pub nodes: Vec<T>,

    /// TTL of the message.
    pub ttl: TimeToLive,
}

/// `SHUFFLE_REPLY` message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShuffleReplyMessage<T> {
    /// The node ID of the message sender.
    pub sender: T,

    /// The nodes selected by `sender` as the reply of the associated `Shuffle` message.
    pub nodes: Vec<T>,
}

/// `DISCONNECT` message.
///
/// This is sent by a node for removing the sender from the active view of the receiver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisconnectMessage<T> {
    /// The node ID of the message sender.
    pub sender: T,

    /// Whether the sender is alive or not.
    ///
    /// If it is `false`, the receiver of the message will remove the sender from its passive view.
    pub alive: bool,
}
