use TimeToLive;

#[derive(Debug, Clone)]
pub enum Message<T> {
    Join {
        sender: T,
    },
    ForwardJoin {
        sender: T,
        new_node: T,
        ttl: TimeToLive,
    },
    Neighbor {
        sender: T,
        high_priority: bool,
    },
    Shuffle {
        sender: T,
        origin: T,
        nodes: Vec<T>,
        ttl: TimeToLive,
    },
    ShuffleReply {
        sender: T,
        nodes: Vec<T>,
    },
}
impl<T> Message<T> {
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
