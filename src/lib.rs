//! A Rust implementation of [HyParView] algorithm.
//!
//! # References
//!
//! - [HyParView: a membership protocol for reliable gossip-based broadcast][HyParView]
//!
//! [HyParView]: http://asc.di.fct.unl.pt/~jleitao/pdf/dsn07-leitao.pdf
#![warn(missing_docs)]
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
mod tests {
    use rand;
    use rand::rngs::ThreadRng;
    use std::collections::HashSet;
    use std::hash::Hash;

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

    #[test]
    fn join_and_leave_works() {
        let mut nodes = vec![
            Node::new("foo", rand::thread_rng()),
            Node::new("bar", rand::thread_rng()),
            Node::new("baz", rand::thread_rng()),
        ];

        // join
        for node in &mut nodes {
            assert!(node.active_view().is_empty());
            assert!(node.passive_view().is_empty());
            node.join("foo");
        }

        execute_actions(&mut nodes);
        for node in &nodes {
            assert_eq!(
                to_set(node.active_view()),
                to_set(["foo", "bar", "baz"].iter().filter(|n| *n != node.id()))
            );
            assert!(node.passive_view().is_empty());
        }

        // leave (alive=true)
        nodes.pop();
        nodes[0].disconnect(&"baz", true);
        nodes[1].disconnect(&"baz", true);
        execute_actions(&mut nodes);

        for node in &nodes {
            assert_eq!(
                to_set(node.active_view()),
                to_set(["foo", "bar"].iter().filter(|n| *n != node.id()))
            );
            assert_eq!(
                to_set(node.passive_view()),
                to_set(["baz"].iter().filter(|n| *n != node.id()))
            );
        }

        // re-join
        nodes.push(Node::new("baz", rand::thread_rng()));
        nodes[2].join("bar");

        execute_actions(&mut nodes);
        for node in &nodes {
            assert_eq!(
                to_set(node.active_view()),
                to_set(["foo", "bar", "baz"].iter().filter(|n| *n != node.id()))
            );
            assert!(node.passive_view().is_empty());
        }

        // leave (alive=false)
        nodes.pop();
        nodes[0].disconnect(&"baz", false);
        nodes[1].disconnect(&"baz", false);
        execute_actions(&mut nodes);

        for node in &nodes {
            assert_eq!(
                to_set(node.active_view()),
                to_set(["foo", "bar"].iter().filter(|n| *n != node.id()))
            );
            assert!(node.passive_view().is_empty());
        }
    }

    #[test]
    fn limit_active_view_size() {
        let options = NodeOptions {
            max_active_view_size: 2,
            active_random_walk_len: 2,
            ..Default::default()
        };
        let mut nodes = vec![
            Node::with_options("foo", rand::thread_rng(), options.clone()),
            Node::with_options("bar", rand::thread_rng(), options.clone()),
            Node::with_options("baz", rand::thread_rng(), options.clone()),
            Node::with_options("qux", rand::thread_rng(), options.clone()),
        ];

        // join
        for node in &mut nodes {
            node.join("foo");
        }

        execute_actions(&mut nodes);
        for node in &nodes {
            assert_eq!(node.active_view().len(), 2);
            let peers = to_set(
                ["foo", "bar", "baz", "qux"]
                    .iter()
                    .filter(|n| *n != node.id()),
            );
            assert!(peers.is_superset(&to_set(node.active_view())));

            if !node.passive_view().is_empty() {
                assert_eq!(
                    to_set(node.passive_view()),
                    peers
                        .difference(&to_set(node.active_view()))
                        .cloned()
                        .collect()
                );
            }
        }
    }

    fn execute_actions(nodes: &mut [Node<&'static str, ThreadRng>]) {
        let mut did_something = true;
        while did_something {
            did_something = false;

            let mut i = 0;
            while i < nodes.len() {
                match rand::random::<usize>() % 30 {
                    0 => nodes[i].fill_active_view(),
                    1 => nodes[i].sync_active_view(),
                    2 => nodes[i].shuffle_passive_view(),
                    _ => {}
                }

                if let Some(action) = nodes[i].poll_action() {
                    did_something = true;
                    match action {
                        Action::Send {
                            destination,
                            message,
                        } => {
                            if let Some(dest) = nodes.iter_mut().find(|n| *n.id() == destination) {
                                dest.handle_protocol_message(message);
                            }
                        }
                        Action::Disconnect { .. } => {}
                        Action::Notify { .. } => {}
                    }
                }
                i += 1;
            }
        }
    }

    fn to_set<I>(iter: I) -> HashSet<I::Item>
    where
        I: IntoIterator,
        I::Item: Eq + Hash,
    {
        iter.into_iter().collect()
    }
}
