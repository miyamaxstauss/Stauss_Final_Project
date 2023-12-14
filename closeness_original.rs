use crate::BFS;
type AdjacencyList = Vec<(u32, Vec<u32>)>;

pub fn closeness_centrality(graph: &AdjacencyList, node: u32) -> f64 {
    // calculate all distances from given node to all other nodes
    let distances: Vec<u32> = BFS::bfs(node as usize, node as usize);
    //println!("distances: {:?}", distances);

    // calculate sum of distances and num of nodes in graph
    let sum_distances: u32 = distances.iter().sum();
    let num_nodes = graph.len() as u32 - 1; // excluding the node itself

    // calculate closeness using formula
    let centrality:f64 = if sum_distances > 0 && num_nodes > 0 {
        num_nodes as f64 / sum_distances as f64
    } else {
        0.0
    };

    // make vector of node, centrality
    let mut node_centrality: Vec<(u32, f64)> = Vec::new();

    //Iterate over all nodes (excluding the given node)
    for i in 0..graph.len() {
        if i as u32 != node {
            // Calculate closeness centrality for each node and store in the vector
            let distances_i: Vec<u32> = BFS::bfs(i, node as usize);
            let sum_distances_i: u32 = distances_i.iter().sum();
            let centrality_i: f64 = if sum_distances_i > 0 {
                (num_nodes as f64 - 1.0) / sum_distances_i as f64
            } else {
                0.0
            };
            node_centrality.push((i as u32, centrality_i));
        }
    }

    // Sort the nodes by centrality in descending order
    node_centrality.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Print the top 5 nodes and their centrality values
    println!("Top 5 Nodes and Closeness Centrality:");
    for (i, (node, centrality)) in node_centrality.iter().take(5).enumerate() {
        println!("{}. Node: {} Closeness Centrality: {}", i + 1, node, centrality);
    }
    return centrality;

}