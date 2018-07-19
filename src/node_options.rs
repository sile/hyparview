/// Options for HyParView [Node](./struct.Node.html).
#[derive(Debug, Clone)]
pub struct NodeOptions {
    /// Maximum number of nodes in the active view.
    pub max_active_view_size: u8,

    /// Maximum number of nodes in the passive view.
    pub max_passive_view_size: u8,

    /// Protocol parameter that is called `ka` in the [paper].
    ///
    /// [paper]: http://asc.di.fct.unl.pt/~jleitao/pdf/dsn07-leitao.pdf
    pub shuffle_active_view_size: u8,

    /// Protocol parameter that is called `kp` in the [paper].
    ///
    /// [paper]: http://asc.di.fct.unl.pt/~jleitao/pdf/dsn07-leitao.pdf
    pub shuffle_passive_view_size: u8,

    /// Protocol parameter that is called `ARWL` in the [paper].
    ///
    /// This is the initial TTL value for `ForwardJoin` and `Shuffle` messages.
    ///
    /// [paper]: http://asc.di.fct.unl.pt/~jleitao/pdf/dsn07-leitao.pdf
    pub active_random_walk_len: u8,

    /// Protocol parameter that is called `PRWL` in the [paper].
    ///
    /// If TTL is equal to the value,
    ///
    /// [paper]: http://asc.di.fct.unl.pt/~jleitao/pdf/dsn07-leitao.pdf
    pub passive_random_walk_len: u8,
}
impl NodeOptions {
    /// The default value of `max_active_view_size` field.
    pub const DEFAULT_MAX_ACTIVE_VIEW_SIZE: u8 = 4;

    /// The default value of `max_passive_view_size` field.
    pub const DEFAULT_MAX_PASSIVE_VIEW_SIZE: u8 = 24;

    /// The default value of `shuffle_active_view_size` field.
    pub const DEFAULT_SHUFFLE_ACTIVE_VIEW_SIZE: u8 = 2;

    /// The default value of `shuffle_passive_view_size` field.
    pub const DEFAULT_SHUFFLE_PASSIVE_VIEW_SIZE: u8 = 2;

    /// The default value of `active_random_walk_len` field.
    pub const DEFAULT_ACTIVE_RANDOM_WALK_LEN: u8 = 5;

    /// The default value of `passive_random_walk_len` field.
    pub const DEFAULT_PASSIVE_RANDOM_WALK_LEN: u8 = 2;
}
impl Default for NodeOptions {
    fn default() -> Self {
        NodeOptions {
            max_active_view_size: Self::DEFAULT_MAX_ACTIVE_VIEW_SIZE,
            max_passive_view_size: Self::DEFAULT_MAX_PASSIVE_VIEW_SIZE,
            shuffle_active_view_size: Self::DEFAULT_SHUFFLE_ACTIVE_VIEW_SIZE,
            shuffle_passive_view_size: Self::DEFAULT_SHUFFLE_PASSIVE_VIEW_SIZE,
            active_random_walk_len: Self::DEFAULT_ACTIVE_RANDOM_WALK_LEN,
            passive_random_walk_len: Self::DEFAULT_PASSIVE_RANDOM_WALK_LEN,
        }
    }
}
