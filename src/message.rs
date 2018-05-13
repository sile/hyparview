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
        sender: T, // TODO: origin
        nodes: Vec<T>,
        ttl: TimeToLive,
    },
    ShuffleReply {
        nodes: Vec<T>,
    },
}
