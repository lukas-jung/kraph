use kraph::{al_graph::ALGraph, Graph, NodeIx};
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

    let txt = std::fs::read_to_string("data/G_100_200.txt").unwrap();
    let mut line_iter = txt.lines();
    let size = line_iter.next().unwrap().parse().unwrap();

    let mut graph: ALGraph<(), NotNan<f64>> = ALGraph::new(size);

    for line in line_iter {
        let mut inner_iter = line.split('\t');
        let from = NodeIx(inner_iter.next().unwrap().parse().unwrap());
        let to = NodeIx(inner_iter.next().unwrap().parse().unwrap());
        let weight: NotNan<f64> = inner_iter.next().unwrap().parse().unwrap();
        graph.add_edge(from, to, weight);
    }
    println!("Graph loaded");

    let kruskal_result = kraph::algo::kruskal(&graph);
    println!("Kruskal: {}", kruskal_result);

    let prim_result = kraph::algo::prim(&graph);
    println!("Prim: {}", prim_result);
}
