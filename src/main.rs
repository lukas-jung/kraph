use kraph::{al_graph::ALGraph, algo::nearest_neighbor, Graph, NodeIx};
use ordered_float::NotNan;

fn main() {
    // let txt = std::fs::read_to_string("data/Graph_ganzganzgross.txt").unwrap();
    // let mut line_iter = txt.lines();
    // let size = line_iter.next().unwrap().parse().unwrap();

    // let mut graph: ALGraph<(), ()> = ALGraph::new(size);

    // for line in line_iter {
    //     let (from, to) = line.split_once('\t').unwrap();
    //     let from = NodeIx(from.parse().unwrap());
    //     let to = NodeIx(to.parse().unwrap());
    //     graph.add_edge(from, to, ());
    // }
    // let count = kraph::algo::count_components(graph);

    // println!("{}", count);

    let txt = std::fs::read_to_string("data/K_12e.txt").unwrap();
    let mut line_iter = txt.lines();
    let size = line_iter.next().unwrap().parse().unwrap();

    // let mut graph: ALGraph<(), NotNan<f64>> = ALGraph::new(size);

    // for line in line_iter {
    //     let mut inner_iter = line.split('\t');
    //     let from = NodeIx(inner_iter.next().unwrap().parse().unwrap());
    //     let to = NodeIx(inner_iter.next().unwrap().parse().unwrap());
    //     let weight: NotNan<f64> = inner_iter.next().unwrap().parse().unwrap();
    //     graph.add_edge(from, to, weight);
    // }
    // println!("Graph loaded");

    // let nn_result = kraph::algo::nearest_neighbor(&graph, NodeIx(0));
    // println!("NN: {}", nn_result);

    // let dmst_result = kraph::algo::double_mst(&graph, NodeIx(0));
    // println!("DMST: {}", dmst_result);

    let mut uint_graph: ALGraph<(), u64> = ALGraph::new(size);
    for line in line_iter {
        let mut inner_iter = line.split('\t');
        let from = NodeIx(inner_iter.next().unwrap().parse().unwrap());
        let to = NodeIx(inner_iter.next().unwrap().parse().unwrap());
        let weight: NotNan<f64> = inner_iter.next().unwrap().parse().unwrap();
        let weight = (weight.into_inner() * 100.0f64).round() as u64;
        uint_graph.add_edge(from, to, weight);
    }

    let bf_result = kraph::algo::brute_force(&uint_graph, NodeIx(0), true) as f64 / 100.0;
    println!("BF: {}", bf_result);
}
