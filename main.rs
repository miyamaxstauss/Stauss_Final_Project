use stauss_final_project::degree;
use stauss_final_project::readfile;
use stauss_final_project::BFS;
use stauss_final_project::closeness_original;
type AdjacencyList = Vec<(u32, Vec<u32>)>;

fn main() {
    // calculating top 10 first degree connections 
    degree::calc_degrees();

    // calculating closeness centrality
    let graph = degree::adjacency_lists();
    let closeness_node = 5;
    closeness_original::closeness_centrality(&graph, closeness_node); 
}