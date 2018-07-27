use rand::{Rng, ThreadRng};
use std::collections::VecDeque;

use message::{
    DisconnectMessage, ForwardJoinMessage, JoinMessage, NeighborMessage, ProtocolMessage,
    ShuffleMessage, ShuffleReplyMessage,
};
use {Action, NodeOptions, TimeToLive};

/// HyParView node.
///
/// # Note on the guarantee of the connectivity of a HyParView cluster
///
/// Because HyParView is a probabilistic algorithm,
/// there is no strong guarantee about the connectivity of the nodes which consist a cluster.
///
/// If the membership of a cluster changes drastically,
/// there is a (usually very low) possibility that the cluster will be splitted to some sub-clusters.
///
/// For recovering the connectivity,
/// an upper layer have to provide some kind of connectivity checking mechanism.
/// And when the cluster division is detected, `Node::join` method should be called in some nodes.
#[derive(Debug)]
pub struct Node<T, R = ThreadRng> {
    id: T,
    actions: VecDeque<Action<T>>,
    active_view: Vec<T>,
    passive_view: Vec<T>,
    rng: R,
    options: NodeOptions,
}
impl<T, R> Node<T, R>
where
    T: Clone + Eq,
    R: Rng,
{
    /// Makes a new `Node` instance with the default options.
    pub fn new(node_id: T, rng: R) -> Self {
        Node::with_options(node_id, rng, NodeOptions::default())
    }

    /// Makes a new `Node` instance with the given options.
    pub fn with_options(node_id: T, rng: R, options: NodeOptions) -> Self {
        Node {
            id: node_id,
            actions: VecDeque::new(),
            active_view: Vec::with_capacity(options.max_active_view_size as usize),
            passive_view: Vec::with_capacity(options.max_passive_view_size as usize),
            rng,
            options,
        }
    }

    /// Returns a reference to the ID of the instance.
    pub fn id(&self) -> &T {
        &self.id
    }

    /// Returns a reference to the active view of the instance.
    pub fn active_view(&self) -> &[T] {
        &self.active_view
    }

    /// Returns a reference to the passive view of the instance.
    pub fn passive_view(&self) -> &[T] {
        &self.passive_view
    }

    /// Returns a reference to the options of the instance.
    pub fn options(&self) -> &NodeOptions {
        &self.options
    }

    /// Returns a mutable reference to the options of the instance.
    pub fn options_mut(&mut self) -> &mut NodeOptions {
        &mut self.options
    }

    /// Starts joining the cluster to which `contact_node_id` belongs.
    ///
    /// This method may be called multiple times for recovering cluster connectivity
    /// if an upper layer detects the cluster is splitted to sub-clusters.
    pub fn join(&mut self, contact_node_id: T) {
        send(
            &mut self.actions,
            contact_node_id,
            ProtocolMessage::join(&self.id),
        );
    }

    /// Removes `node` from the active view of the instance.
    ///
    /// If there is no such node, it is simply ignored.
    ///
    /// If the active view is not full, a node randomly selected from the passive view
    /// will be promoted to the active view if possible.
    ///
    /// This is equivalent to the following code:
    /// ```norun
    /// let message = ProtocolMessage::Disconnect(DisconnectMessage{sender: node.clone(), alive});
    /// self.handle_protocol_message(message);
    /// ```
    pub fn disconnect(&mut self, node: &T, alive: bool) {
        self.handle_protocol_message(ProtocolMessage::disconnect(node, alive));
    }

    /// Handles the given incoming message.
    pub fn handle_protocol_message(&mut self, message: ProtocolMessage<T>) {
        let sender = message.sender().clone();
        match message {
            ProtocolMessage::Join(m) => self.handle_join(m),
            ProtocolMessage::ForwardJoin(m) => self.handle_forward_join(m),
            ProtocolMessage::Neighbor(m) => self.handle_neighbor(m),
            ProtocolMessage::Shuffle(m) => self.handle_shuffle(m),
            ProtocolMessage::ShuffleReply(m) => self.handle_shuffle_reply(m),
            ProtocolMessage::Disconnect(m) => {
                self.handle_disconnect(m);
                return;
            }
        }
        self.disconnect_unless_active_view_node(sender);
    }

    /// Starts shuffling the passive view of the instance.
    ///
    /// This method should be invoked periodically to keep the passive view fresh.
    pub fn shuffle_passive_view(&mut self) {
        if let Some(node) = self.select_random_from_active_view() {
            self.rng.shuffle(&mut self.passive_view);
            self.rng.shuffle(&mut self.active_view);

            let pv_size = self.options.shuffle_passive_view_size as usize;
            let av_size = self.options.shuffle_active_view_size as usize;
            let shuffle_size = 1 + pv_size + av_size;

            let mut nodes = Vec::with_capacity(shuffle_size);
            nodes.extend(self.passive_view.iter().take(pv_size).cloned());
            nodes.extend(self.active_view.iter().take(av_size).cloned());
            nodes.push(self.id.clone());

            let ttl = TimeToLive::new(self.options.active_random_walk_len);
            let message = ProtocolMessage::shuffle(&self.id, self.id.clone(), nodes, ttl);
            send(&mut self.actions, node, message);
        }
    }

    /// Promotes a node from the passive view to the active view if the latter is not full.
    ///
    /// This method should be invoked periodically to keep the active view full.
    pub fn fill_active_view(&mut self) {
        if !self.is_active_view_full() {
            if let Some(node) = self.select_random_from_passive_view() {
                let high_priority = self.active_view.is_empty();
                let message = ProtocolMessage::neighbor(&self.id, high_priority);
                send(&mut self.actions, node, message);
            }
        }
    }

    /// Sends `NEIGHBOR` message to the members of the active view for
    /// maintaining the symmetry property of the view.
    ///
    /// This method should be invoked periodically to keep the symmetry property of the active view.
    pub fn sync_active_view(&mut self) {
        for node in self.active_view.clone() {
            let message = ProtocolMessage::neighbor(&self.id, false);
            send(&mut self.actions, node, message);
        }
    }

    /// Polls the next action that the node wants to execute.
    ///
    /// For running the HyParView node correctly,
    /// this method must be called periodically and the resulting action must be executed by the caller.
    pub fn poll_action(&mut self) -> Option<Action<T>> {
        self.actions.pop_front()
    }

    fn is_active_view_full(&self) -> bool {
        self.active_view.len() >= self.options.max_active_view_size as usize
    }

    fn is_passive_view_full(&self) -> bool {
        self.passive_view.len() >= self.options.max_passive_view_size as usize
    }

    fn handle_join(&mut self, m: JoinMessage<T>) {
        let new_node = m.sender;
        self.add_to_active_view(new_node.clone(), true);
        let ttl = TimeToLive::new(self.options.active_random_walk_len);
        for n in self.active_view.iter().filter(|n| **n != new_node) {
            let message = ProtocolMessage::forward_join(&self.id, new_node.clone(), ttl);
            send(&mut self.actions, n.clone(), message);
        }
    }

    fn handle_forward_join(&mut self, m: ForwardJoinMessage<T>) {
        if m.ttl.is_expired() || self.active_view.is_empty() {
            self.add_to_active_view(m.new_node, true);
        } else {
            if m.ttl.as_u8() == self.options.passive_random_walk_len {
                self.add_to_passive_view(m.new_node.clone());
            }
            if let Some(next) = self.select_forwarding_destination(&[&m.sender]) {
                let message =
                    ProtocolMessage::forward_join(&self.id, m.new_node, m.ttl.decrement());
                send(&mut self.actions, next, message);
            } else {
                self.add_to_active_view(m.new_node, true);
            }
        }
    }

    fn handle_neighbor(&mut self, m: NeighborMessage<T>) {
        if m.high_priority || !self.is_active_view_full() {
            self.add_to_active_view(m.sender, false);
        }
    }

    fn handle_shuffle(&mut self, m: ShuffleMessage<T>) {
        if m.ttl.is_expired() {
            self.rng.shuffle(&mut self.passive_view);
            let reply_nodes = self.passive_view
                .iter()
                .take(m.nodes.len())
                .cloned()
                .collect();
            let message = ProtocolMessage::shuffle_reply(&self.id, reply_nodes);
            send(&mut self.actions, m.origin.clone(), message);
            self.add_shuffled_nodes_to_passive_view(m.nodes);
        } else if let Some(destination) =
            self.select_forwarding_destination(&[&m.origin, &m.sender])
        {
            let message = ProtocolMessage::shuffle(&self.id, m.origin, m.nodes, m.ttl.decrement());
            send(&mut self.actions, destination, message);
        }
    }

    fn handle_shuffle_reply(&mut self, m: ShuffleReplyMessage<T>) {
        self.add_shuffled_nodes_to_passive_view(m.nodes);
    }

    fn handle_disconnect(&mut self, m: DisconnectMessage<T>) {
        if self.remove_from_active_view(&m.sender) {
            self.remove_from_passive_view(&m.sender);
            self.fill_active_view();
        }
        if m.alive {
            self.add_to_passive_view(m.sender);
        }
    }

    fn add_shuffled_nodes_to_passive_view(&mut self, nodes: Vec<T>) {
        for n in nodes {
            self.add_to_passive_view(n);
        }
    }

    fn add_to_active_view(&mut self, node: T, high_priority: bool) {
        if self.active_view.contains(&node) || node == self.id {
            return;
        }
        self.remove_random_from_active_view_if_full();
        self.remove_from_passive_view(&node);
        self.active_view.push(node.clone());
        send(
            &mut self.actions,
            node.clone(),
            ProtocolMessage::neighbor(&self.id, high_priority),
        );
        self.actions.push_back(Action::notify_up(node));
    }

    fn add_to_passive_view(&mut self, node: T) {
        if self.active_view.contains(&node) || self.passive_view.contains(&node) || node == self.id
        {
            return;
        }
        self.remove_random_from_passive_view_if_full();
        self.passive_view.push(node);
    }

    fn remove_from_active_view(&mut self, node: &T) -> bool {
        let index = self.active_view.iter().position(|n| n == node);
        if let Some(i) = index {
            self.remove_from_active_view_by_index(i);
            true
        } else {
            false
        }
    }

    fn remove_from_active_view_by_index(&mut self, i: usize) {
        let node = self.active_view.swap_remove(i);
        send(
            &mut self.actions,
            node.clone(),
            ProtocolMessage::disconnect(&self.id, true),
        );
        self.actions.push_back(Action::disconnect(node.clone()));
        self.actions.push_back(Action::notify_down(node.clone()));
        self.add_to_passive_view(node);
    }

    fn remove_random_from_active_view_if_full(&mut self) {
        if self.is_active_view_full() {
            let i = self.rng.gen_range(0, self.active_view.len());
            self.remove_from_active_view_by_index(i);
        }
    }

    fn remove_from_passive_view(&mut self, node: &T) {
        let position = self.passive_view.iter().position(|n| n == node);
        if let Some(i) = position {
            self.passive_view.swap_remove(i);
        }
    }

    fn remove_random_from_passive_view_if_full(&mut self) {
        if self.is_passive_view_full() {
            let i = self.rng.gen_range(0, self.passive_view.len());
            self.passive_view.swap_remove(i);
        }
    }

    fn disconnect_unless_active_view_node(&mut self, node: T) {
        if !self.active_view.contains(&node) && self.id != node {
            send(
                &mut self.actions,
                node.clone(),
                ProtocolMessage::disconnect(&self.id, true),
            );
            self.actions.push_back(Action::disconnect(node));
        }
    }

    fn select_forwarding_destination(&mut self, excludes: &[&T]) -> Option<T> {
        let mut i = 0;
        let mut tail = self.active_view.len();
        while i < tail && tail != 0 {
            let is_not_candidate = excludes.contains(&&self.active_view[i]);
            if is_not_candidate {
                self.active_view.swap(i, tail - 1);
                tail -= 1;
            } else {
                i += 1;
            }
        }

        if tail == 0 {
            None
        } else {
            let i = self.rng.gen_range(0, tail);
            Some(self.active_view[i].clone())
        }
    }

    fn select_random_from_active_view(&mut self) -> Option<T> {
        if self.active_view.is_empty() {
            None
        } else {
            let i = self.rng.gen_range(0, self.active_view.len());
            Some(self.active_view[i].clone())
        }
    }

    fn select_random_from_passive_view(&mut self) -> Option<T> {
        if self.passive_view.is_empty() {
            None
        } else {
            let i = self.rng.gen_range(0, self.passive_view.len());
            Some(self.passive_view[i].clone())
        }
    }
}

fn send<T>(actions: &mut VecDeque<Action<T>>, destination: T, message: ProtocolMessage<T>) {
    actions.push_back(Action::send(destination, message));
}
