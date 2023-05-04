use std::rc::Rc;

pub mod al_graph;
pub mod algo;

pub trait Graph<N, E> {
    fn add_edge(&mut self, from: u32, to: u32, weight: E);
    fn get_edges(&self, node_ix: u32) -> &[(u32, Rc<E>)];
    fn get_node(&self, node_ix: u32) -> Option<&N>;
    fn get_node_mut(&mut self, node_ix: u32) -> Option<&mut N>;
    fn size(&self) -> u32;
}
