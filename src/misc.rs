use Message;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimeToLive(u8);
impl TimeToLive {
    pub fn new(ttl: u8) -> Self {
        TimeToLive(ttl)
    }

    pub fn as_u8(self) -> u8 {
        self.0
    }

    pub fn is_expired(&self) -> bool {
        self.0 == 0
    }

    pub(crate) fn decrement(self) -> Self {
        TimeToLive(self.0.saturating_sub(1))
    }
}

#[derive(Debug)]
pub enum Action<T> {
    Send { destination: T, message: Message<T> },
    Disconnect { node: T },
    Notify { event: Event<T> },
}

#[derive(Debug, Clone)]
pub enum Event<T> {
    NeighborUp { node: T },
    NeighborDown { node: T },
}
