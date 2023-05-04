use crate::Graph;

pub fn count_components<N, E>(graph: impl Graph<N, E>) -> u32 {
    let size = graph.size();
    let mut visited = vec![false; size as usize];
    let mut count = 0;

    for ix in 0..size {
        if !std::mem::replace(&mut visited[ix as usize], true) {
            let mut stack = vec![ix];

            while let Some(current) = stack.pop() {
                for &(adj_ix, _) in graph.get_edges(current) {
                    if !std::mem::replace(&mut visited[adj_ix as usize], true) {
                        stack.push(adj_ix);
                    }
                }
            }

            count += 1;
        }
    }
    count
}
