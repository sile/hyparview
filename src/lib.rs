//! A Rust implementation of [HyParView] algorithm.
//!
//! # References
//!
//! - [HyParView: a membership protocol for reliable gossip-based broadcast][HyParView]
//!
//! [HyParView]: http://asc.di.fct.unl.pt/~jleitao/pdf/dsn07-leitao.pdf
#![warn(missing_docs)]
extern crate rand;

pub use action::Action;
pub use event::Event;
pub use node::Node;
pub use node_options::NodeOptions;
pub use ttl::TimeToLive;

mod action;
mod event;
mod node;
mod node_options;
mod ttl;

pub mod message;

#[cfg(test)]
mod test {
    use rand;

    use super::message::ProtocolMessage;
    use super::*;

    macro_rules! assert_some {
        ($e:expr) => {
            if let Some(x) = $e {
                x
            } else {
                panic!("{:?} must be `Some(_)`", stringify!($e));
            }
        };
    }

    #[test]
    fn single_join_works() {
        let mut node = Node::new("foo", rand::thread_rng());
        node.join("bar");

        let action = assert_some!(node.poll_action());
        assert_eq!(action, Action::send("bar", ProtocolMessage::join(&"foo")));
        assert!(node.poll_action().is_none());
    }
}
