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
