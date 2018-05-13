#[derive(Debug, Clone, Copy)]
pub struct TimeToLive(pub u8);
impl TimeToLive {
    pub fn is_expired(&self) -> bool {
        self.0 == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    High,
    Low,
}

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
        priority: Priority,
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
