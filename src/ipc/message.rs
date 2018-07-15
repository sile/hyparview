use ipc::TimeToLive;

/// Messages used for inter-process communication.
///
/// Unlike the [paper][HyParView], this does not contain `DISCONNECT` message.
/// In this crate, disconnections are assumed to be handled at the out of the normal messages
/// (e.g., Disconnections at the TCP level may be used for that purpose).
///
/// [HyParView]: http://asc.di.fct.unl.pt/~jleitao/pdf/dsn07-leitao.pdf
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message<T> {
    /// `JOIN` message.
    Join(JoinMessage<T>),

    /// `FORWARD_JOIN` message.
    ForwardJoin(ForwardJoinMessage<T>),

    /// `NEIGHBOR` message.
    Neighbor(NeighborMesssage<T>),

    /// `SHUFFLE` message.
    Shuffle(ShuffleMessage<T>),

    /// `SHUFFLE_REPLY` message.
    ShuffleReply(ShuffleReplyMessage<T>),
}
impl<T> Message<T> {
    /// Returns the node ID of the sender of the message.
    pub fn sender(&self) -> &T {
        match self {
            Message::Join(m) => &m.sender,
            Message::ForwardJoin(m) => &m.sender,
            Message::Neighbor(m) => &m.sender,
            Message::Shuffle(m) => &m.sender,
            Message::ShuffleReply(m) => &m.sender,
        }
    }
}
impl<T: Clone> Message<T> {
    pub(crate) fn join(sender: &T) -> Self {
        Message::Join(JoinMessage {
            sender: sender.clone(),
        })
    }

    pub(crate) fn forward_join(sender: &T, new_node: T, ttl: TimeToLive) -> Self {
        Message::ForwardJoin(ForwardJoinMessage {
            sender: sender.clone(),
            new_node,
            ttl,
        })
    }

    pub(crate) fn neighbor(sender: &T, high_priority: bool) -> Self {
        Message::Neighbor(NeighborMesssage {
            sender: sender.clone(),
            high_priority,
        })
    }

    pub(crate) fn shuffle(sender: &T, origin: T, nodes: Vec<T>, ttl: TimeToLive) -> Self {
        Message::Shuffle(ShuffleMessage {
            sender: sender.clone(),
            origin,
            nodes,
            ttl,
        })
    }

    pub(crate) fn shuffle_reply(sender: &T, nodes: Vec<T>) -> Self {
        Message::ShuffleReply(ShuffleReplyMessage {
            sender: sender.clone(),
            nodes,
        })
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
pub struct NeighborMesssage<T> {
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
