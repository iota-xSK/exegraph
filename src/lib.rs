#![no_std]
use core::array;

pub struct Node<
    P: Processor<D, MAX_IN, MAX_OUT>,
    D: Default,
    const MAX_IN: usize,
    const MAX_OUT: usize,
> {
    pub processor: Option<P>,
    pub read: [D; MAX_OUT],
    pub write: [D; MAX_OUT],
}

impl<P: Processor<D, MAX_IN, MAX_OUT>, D: Default, const MAX_IN: usize, const MAX_OUT: usize>
    Node<P, D, MAX_IN, MAX_OUT>
{
    pub fn new(processor: Option<P>) -> Self {
        Self {
            processor,
            read: array::from_fn(|_| D::default()),
            write: array::from_fn(|_| D::default()),
        }
    }
}

pub trait Processor<D, const MAX_IN: usize, const MAX_OUT: usize> {
    fn process(&mut self, inputs: &[Option<&D>; MAX_IN]) -> [D; MAX_OUT];
}

pub struct StaticGraph<
    P: Processor<D, MAX_IN, MAX_OUT>,
    D: Default,
    const MAX_IN: usize,
    const MAX_OUT: usize,
    const MAX_NODES: usize,
> {
    pub nodes: [Node<P, D, MAX_IN, MAX_OUT>; MAX_NODES],
    used: [bool; MAX_NODES],
    connections: [[Option<(usize, usize)>; MAX_IN]; MAX_NODES],
}

pub struct NodeHandle(usize);

impl<
        P: Processor<D, MAX_IN, MAX_OUT>,
        D: Default,
        const MAX_IN: usize,
        const MAX_OUT: usize,
        const MAX_NODES: usize,
    > StaticGraph<P, D, MAX_IN, MAX_OUT, MAX_NODES>
{
    pub fn new() -> Self {
        Self {
            nodes: array::from_fn(|_| Node::new(None)),
            used: [false; MAX_NODES],
            connections: [[None; MAX_IN]; MAX_NODES],
        }
    }
    pub fn process(&mut self) {
        for i in 0..MAX_NODES {
            if self.used[i] {
                let mut inputs = array::from_fn(|_| None);
                for (j, input) in self.connections[i].iter().enumerate() {
                    if let Some((in_node, in_port)) = input {
                        inputs[j] = Some(&self.nodes[*in_node].read[*in_port]);
                    }
                }
                if let Some(ref mut processor) = self.nodes[i].processor {
                    self.nodes[i].write = processor.process(&inputs);
                }
            }
        }
        for i in 0..MAX_NODES {
            core::mem::swap(&mut self.nodes[i].read, &mut self.nodes[i].write)
        }
    }

    pub fn add_node(&mut self, processor: P) -> Option<NodeHandle> {
        for i in 0..MAX_NODES {
            if !self.used[i] {
                self.nodes[i].processor = Some(processor);
                self.used[i] = true;
                return Some(NodeHandle(i));
            }
        }
        return None;
    }

    pub fn remove_node(&mut self, idx: NodeHandle) {
        let idx = idx.0;
        self.used[idx] = false;

        for i in self.connections.iter_mut() {
            for j in i.iter_mut() {
                if let Some((in_node, _)) = *j {
                    if in_node == idx {
                        *j = None;
                    }
                }
            }
        }
    }
    pub fn set_edge(
        &mut self,
        to: &NodeHandle,
        to_sink: usize,
        from: &NodeHandle,
        from_source: usize,
    ) {
        self.connections[to.0][to_sink] = Some((from.0, from_source));
    }
    pub fn unset_edge(&mut self, to: &NodeHandle, to_sink: usize) {
        self.connections[to.0][to_sink] = None;
    }
}
