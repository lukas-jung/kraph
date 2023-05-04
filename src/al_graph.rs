use std::{collections::HashMap, rc::Rc};

use crate::Graph;

pub struct ALGraph<N, E> {
    adj_lists: HashMap<u32, Vec<(u32, Rc<E>)>>,
    nodes: Vec<N>,
}

impl<N, E> ALGraph<N, E>
where
    N: Default + Clone,
{
    pub fn new(size: u32) -> Self {
        ALGraph {
            adj_lists: Default::default(),
            nodes: vec![Default::default(); size as usize],
        }
    }
}

impl<N, E> Graph<N, E> for ALGraph<N, E> {
    fn add_edge(&mut self, from: u32, to: u32, weight: E) {
        if from >= self.size() {
            panic!("from index out of bounds")
        }
        if to >= self.size() {
            panic!("to index out of bounds")
        }

        let weight = Rc::new(weight);
        self.adj_lists
            .entry(from)
            .or_default()
            .push((to, weight.clone()));

        self.adj_lists.entry(to).or_default().push((from, weight));
    }

    fn get_edges(&self, node_ix: u32) -> &[(u32, Rc<E>)] {
        self.adj_lists
            .get(&node_ix)
            .map(|al| al.as_slice())
            .unwrap_or_default()
    }

    fn get_all_edges(&self) -> Box<dyn Iterator<Item = (u32, u32, &E)> + '_> {
        Box::new(self.adj_lists.iter().flat_map(|(from_ix, edges)| {
            edges.iter().filter_map(move |(to_ix, edge)| {
                if from_ix <= to_ix {
                    Some((*from_ix, *to_ix, &**edge))
                } else {
                    None
                }
            })
        }))
    }

    fn get_node(&self, node_ix: u32) -> Option<&N> {
        self.nodes.get(node_ix as usize)
    }

    fn get_node_mut(&mut self, node_ix: u32) -> Option<&mut N> {
        self.nodes.get_mut(node_ix as usize)
    }

    fn size(&self) -> u32 {
        self.nodes.len() as u32
    }
}
