use rand::{self, Rng, ThreadRng};
use std::collections::VecDeque;

use {Action, Event, Message, TimeToLive};

#[derive(Debug)]
pub struct Node<T, R = ThreadRng> {
    id: T,
    rng: R,
    actions: VecDeque<Action<T>>,
    active_view: Vec<T>,
    passive_view: Vec<T>,
    max_active_view_size: u8,
    max_passive_view_size: u8,
    active_random_walk_len: u8,
    passive_random_walk_len: u8,
}
impl<T> Node<T, ThreadRng>
where
    T: Clone + PartialEq,
{
    pub fn new(node_id: T) -> Self {
        Node::with_rng(node_id, rand::thread_rng())
    }
}
impl<T, R> Node<T, R>
where
    T: Clone + PartialEq,
    R: Rng,
{
    // TODO: builder
    pub fn with_rng(node_id: T, rng: R) -> Self {
        Node {
            id: node_id,
            rng,
            actions: VecDeque::new(),
            active_view: Vec::new(),
            passive_view: Vec::new(),
            max_active_view_size: 4,
            max_passive_view_size: 24,
            active_random_walk_len: 5,
            passive_random_walk_len: 2,
        }
    }

    pub fn join(&mut self, contact_node_id: T) {
        let message = Message::Join {
            sender: self.id.clone(),
        };
        self.send_message(contact_node_id, message);
    }

    pub fn disconnect(&mut self, node: T) {
        self.remove_from_active_view(&node);
        // TODO: send NEIGHBOR if needed
    }

    pub fn handle_message(&mut self, message: Message<T>) {
        match message {
            Message::Join { sender } => self.handle_join(sender),
            Message::ForwardJoin {
                sender,
                new_node,
                ttl,
            } => self.handle_forward_join(sender, new_node, ttl),
            Message::Neighbor {
                sender,
                high_priority,
            } => self.handle_neighbor(sender, high_priority),
            Message::Shuffle { sender, nodes, ttl } => self.handle_shuffle(sender, nodes, ttl),
            Message::ShuffleReply { nodes } => self.handle_shuffle_reply(nodes),
        }
    }

    pub fn shuffle_passive_view(&mut self) {}
    pub fn fill_active_view(&mut self) {}

    pub fn poll_action(&mut self) -> Option<Action<T>> {
        self.actions.pop_front()
    }

    pub fn is_active_view_full(&self) -> bool {
        self.active_view.len() == self.max_active_view_size as usize
    }

    pub fn is_passive_view_full(&self) -> bool {
        self.passive_view.len() == self.max_passive_view_size as usize
    }

    fn handle_join(&mut self, new_node: T) {
        self.remove_from_active_view(&new_node);
        if self.is_active_view_full() {
            self.drop_random_element_from_active_view();
        }

        // TODO: remove clone
        for n in self.active_view.clone() {
            let message = Message::ForwardJoin {
                sender: self.id.clone(),
                new_node: new_node.clone(),
                ttl: TimeToLive::new(self.active_random_walk_len),
            };
            self.send_message(n, message);
        }
        self.add_to_active_view(new_node);
    }

    fn handle_forward_join(&mut self, sender: T, new_node: T, ttl: TimeToLive) {
        if ttl.is_expired() || self.active_view.is_empty() {
            self.remove_from_active_view(&new_node); // TODO: 共通化
            if self.is_active_view_full() {
                self.drop_random_element_from_active_view();
            }
            self.add_to_active_view(new_node);
        } else {
            if ttl.as_u8() == self.passive_random_walk_len {
                self.add_to_passive_view(new_node.clone());
            }
            if let Some(destination) = self.select_forwarding_destination(&sender) {
                let message = Message::ForwardJoin {
                    sender: self.id.clone(),
                    new_node,
                    ttl: ttl.decrement(),
                };
                self.send_message(destination, message);
            }
        }
    }

    fn select_forwarding_destination(&mut self, sender: &T) -> Option<T> {
        let position = self.active_view.iter().position(|n| n == sender);
        let max = if let Some(i) = position {
            let j = self.active_view.len() - 1;
            self.active_view.swap(i, j);
            j
        } else {
            self.active_view.len()
        };
        if max == 0 {
            None
        } else {
            let i = self.rng.gen_range(0, max);
            Some(self.active_view[i].clone())
        }
    }

    fn handle_neighbor(&mut self, sender: T, high_priority: bool) {
        if self.active_view.iter().find(|n| **n == sender).is_some() {
            return;
        }

        if high_priority {
            if self.is_active_view_full() {
                self.drop_random_element_from_active_view();
            }
            self.add_to_active_view(sender);
        } else if self.is_active_view_full() {
            self.actions.push_back(Action::Disconnect { node: sender });
        } else {
            self.add_to_active_view(sender);
        }
    }

    fn handle_shuffle(&mut self, sender: T, nodes: Vec<T>, ttl: TimeToLive) {
        if ttl.is_expired() {
            self.rng.shuffle(&mut self.passive_view);
            let reply_nodes = self.passive_view
                .iter()
                .take(nodes.len())
                .cloned()
                .collect();
            let message = Message::ShuffleReply { nodes: reply_nodes };
            self.send_message(sender, message);

            for n in nodes {
                self.add_to_passive_view(n);
            }
        } else {
            if let Some(destination) = self.select_forwarding_destination(&sender) {
                let message = Message::Shuffle {
                    sender,
                    nodes,
                    ttl: ttl.decrement(),
                };
                self.send_message(destination, message);
            }
        }
    }

    fn handle_shuffle_reply(&mut self, nodes: Vec<T>) {
        for n in nodes {
            self.add_to_passive_view(n);
        }
    }

    fn send_message(&mut self, destination: T, message: Message<T>) {
        let action = Action::Send {
            destination,
            message,
        };
        self.actions.push_back(action);
    }

    fn add_to_active_view(&mut self, node: T) {
        // Assumes the active view does not contain `node`. (TODO)
        let message = Message::Neighbor {
            sender: self.id.clone(),
            high_priority: true,
        };
        self.send_message(node.clone(), message);

        self.remove_from_passive_view(&node);
        self.active_view.push(node.clone());
        self.actions.push_back(Action::Notify {
            event: Event::NeighborUp { node },
        });
    }

    fn add_to_passive_view(&mut self, node: T) {
        // Assumes the passive view does not contain `node`. (TODO)
        if self.is_passive_view_full() {
            self.drop_random_element_from_passive_view();
        }
        self.passive_view.push(node.clone());
    }

    fn remove_from_active_view(&mut self, node: &T) {
        let position = self.active_view.iter().position(|n| n == node);
        if let Some(i) = position {
            self.active_view.swap_remove(i);
            self.actions.push_back(Action::Notify {
                event: Event::NeighborDown { node: node.clone() },
            });
            self.actions
                .push_back(Action::Disconnect { node: node.clone() });
            self.add_to_passive_view(node.clone());
        }
    }

    fn remove_from_passive_view(&mut self, node: &T) {
        let position = self.passive_view.iter().position(|n| n == node);
        if let Some(i) = position {
            self.passive_view.swap_remove(i);
        }
    }

    fn drop_random_element_from_active_view(&mut self) {
        let i = self.rng.gen_range(0, self.active_view.len());
        self.active_view.swap_remove(i);
    }

    fn drop_random_element_from_passive_view(&mut self) {
        let i = self.rng.gen_range(0, self.passive_view.len());
        self.passive_view.swap_remove(i);
    }
}
