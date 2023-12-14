
// Below are all helper functions to calculate the top 10 most connected nodes & degrees 
use crate::readfile;
use std::cmp;
use std::collections::HashSet;

// This function returns a list of all neighbhors from nodes
pub fn neighbors(point:u32) -> Vec<u32> {
let mut neighbors_list: Vec<u32> = Vec::new();
let edges:Vec<(u32, u32)> = unique_nodes().1; 
    for i in 0..edges.len(){
        // pushing neighbors of x values into neighbors_list
        if point == edges[i].0{
            neighbors_list.push(edges[i].1);
        }
        // pushing neighbors of y values into neighbors_list
         if point == edges[i].1{
            neighbors_list.push(edges[i].0);
        }
    }
    neighbors_list.dedup();
    //println!("{:?}", neighbors_list);
    return neighbors_list;
}

// This helper function finds all the unique nodes in the dataset with no duplicates
pub fn unique_nodes() -> (Vec<u32>, Vec<(u32, u32)>) {
    let mut vector: Vec<u32> = Vec::new();
    // Read through dataset and calls on read_file function 
    let test = readfile::read_file("disease_dataset.txt");
        for i in 0..test.len(){
            vector.push(test[i].0); 
            vector.push(test[i].1);
        }
        vector.sort();
        vector.dedup();
        //println!("{:?}, {:?}", vector, test);
        return (vector, test); 
    }


// This function prints all (node, neighbors) from the dataset
pub fn adjacency_lists() -> Vec<(u32, Vec<u32>)> {
    let nodes = unique_nodes().0;
    let mut used_nodes: Vec<(u32)> = Vec::new();
    let mut degree: Vec<(u32, Vec<u32>)> = Vec::new();
    let mut number_of_neighbors: Vec<u32> = Vec::new() ;
    // Iterates through unique nodes in graph & finds degree of each unique node
    for i in 0..nodes.len(){
        if used_nodes.contains(&nodes[i]){
            break;
        }
    else{
        number_of_neighbors = neighbors(nodes[i]);
        degree.push((nodes[i], number_of_neighbors));
        used_nodes.push(nodes[i]);
        }
    }
    degree.sort();
    degree.dedup(); 
    /* 
    // Sorts, removes duplicates, and prints all unique (nodes, neighbors)
    for (node, neighbors) in &degree{
        println!("node {}: {:?}", node, neighbors);
    }
    */
        return degree;
    
    }


// This function calculate top 10 degrees & shows connectivity
pub fn calc_degrees()  {
    // Calling unique_nodes() to pass through dataset 
    let nodes: Vec<u32> = unique_nodes().0;
    let mut degree: Vec<(u32, usize)> = Vec::new();
    let mut number_of_neighbors: usize;
    // Iterates through unique nodes in graph & finds degree of each unique node
    for i in 0..nodes.len(){
         number_of_neighbors = neighbors(nodes[i]).len();
         degree.push((nodes[i], number_of_neighbors));
    }
    degree.sort_by(|a, b| b.1.cmp(&a.1));
    degree.dedup(); 
    // Sort, remove duplicates, print out top 10 degrees 
    println!("Top 10 Nodes and Degree Centrality:");
    for i in 0..10{
        println!("degree {:?}", degree[i]);
    }
}










