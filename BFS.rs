use std::collections::VecDeque;
use crate::degree;
type AdjacencyList = Vec<(u32, Vec<u32>)>;
use std::collections::HashMap;


// This function calculates all BFS for dataset (takes in a start and end node and returns shortest path)
pub fn bfs(start_node:usize, end_node:usize) -> Vec<u32> { 
    let mut shortest_paths: Vec<u32> = Vec::new();
    //let mut shortest_paths: Vec<usize> = Vec::new();
    let n: usize = degree::unique_nodes().0.len() + 1; 
    // Crates visited, previous, and queue to keep track of node going through graph
    let mut visited_nodes: Vec<bool> = vec![false;n];
    let mut previous_nodes: Vec<usize> = vec![100000; n];
    let mut queue = VecDeque::new();

    queue.push_back(start_node); 
    visited_nodes[start_node] = true;
// continues until queue is empty, use current node to update 
    while queue.is_empty() == false  { 
        let current_node: usize = queue.pop_front().unwrap();
        visited_nodes[current_node] = true;
        if current_node == end_node{
            break;
        }
        for i in degree::neighbors(current_node as u32){
            if visited_nodes[i as usize] == false{ 
                visited_nodes[i as usize] = true;
                previous_nodes[i as usize] = current_node;
                queue.push_back(i as usize);
            }
        }
    }
    let mut trace_back: usize = end_node as usize;
    while trace_back != start_node{
        if trace_back < previous_nodes.len() {
            if trace_back != end_node {
                //shortest_paths.push(trace_back)
                shortest_paths.push(trace_back as u32);
            }
            trace_back = previous_nodes[trace_back];
        } else {
            break;
        }
    }
    shortest_paths.reverse();
    //println!("{:?}", shortest_paths);
    // Only returns path in between start & end node (nodes that have been traversed)
    return shortest_paths;
}



