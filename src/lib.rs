//! A Rust implementation of [HyParView] algorithm.
//!
//! # Examples
//!
//! TODO
//!
//! # References
//!
//! - [HyParView: a membership protocol for reliable gossip-based broadcast][HyParView]
//!
//! [HyParView]: http://asc.di.fct.unl.pt/~jleitao/pdf/dsn07-leitao.pdf
#![warn(missing_docs)]
extern crate rand;

pub use message::Message;
pub use misc::{Action, Event, TimeToLive};
pub use node::Node;
pub use node_options::NodeOptions;

mod message;
mod misc;
mod node;
mod node_options;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut node = Node::new("foo");
        node.join("bar");
    }
}
