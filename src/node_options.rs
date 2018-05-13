use rand::{self, Rng, ThreadRng};

#[derive(Debug, Clone)]
pub struct NodeOptions<R> {
    pub rng: R,
    pub max_active_view_size: u8,
    pub max_passive_view_size: u8,
    pub shuffle_active_view_size: u8,
    pub shuffle_passive_view_size: u8,
    pub active_random_walk_len: u8,
    pub passive_random_walk_len: u8,
}
impl NodeOptions<ThreadRng> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<R: Rng> NodeOptions<R> {
    pub const DEFAULT_MAX_ACTIVE_VIEW_SIZE: u8 = 4;
    pub const DEFAULT_MAX_PASSIVE_VIEW_SIZE: u8 = 24;
    pub const DEFAULT_SHUFFLE_ACTIVE_VIEW_SIZE: u8 = 2;
    pub const DEFAULT_SHUFFLE_PASSIVE_VIEW_SIZE: u8 = 2;
    pub const DEFAULT_ACTIVE_RANDOM_WALK_LEN: u8 = 5;
    pub const DEFAULT_PASSIVE_RANDOM_WALK_LEN: u8 = 2;

    pub fn set_rng<S: Rng>(self, rng: S) -> NodeOptions<S> {
        NodeOptions {
            rng,
            max_active_view_size: self.max_active_view_size,
            max_passive_view_size: self.max_passive_view_size,
            shuffle_active_view_size: self.shuffle_active_view_size,
            shuffle_passive_view_size: self.shuffle_passive_view_size,
            active_random_walk_len: self.active_random_walk_len,
            passive_random_walk_len: self.passive_random_walk_len,
        }
    }
}
impl Default for NodeOptions<ThreadRng> {
    fn default() -> Self {
        NodeOptions {
            rng: rand::thread_rng(),
            max_active_view_size: Self::DEFAULT_MAX_ACTIVE_VIEW_SIZE,
            max_passive_view_size: Self::DEFAULT_MAX_PASSIVE_VIEW_SIZE,
            shuffle_active_view_size: Self::DEFAULT_SHUFFLE_ACTIVE_VIEW_SIZE,
            shuffle_passive_view_size: Self::DEFAULT_SHUFFLE_PASSIVE_VIEW_SIZE,
            active_random_walk_len: Self::DEFAULT_ACTIVE_RANDOM_WALK_LEN,
            passive_random_walk_len: Self::DEFAULT_PASSIVE_RANDOM_WALK_LEN,
        }
    }
}
