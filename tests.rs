#[cfg(test)]
use crate::degree;
use crate::readfile;
use crate::BFS;


pub mod tests {
    use super::*;
    
    #[test] // test case 1: calculating bfs
    pub fn test_bfs() {
        assert_eq!(BFS::bfs(72,747), [360, 466, 623, 909, 859, 482, 351, 387, 710, 815, 869, 827, 279, 566, 131]);
    }
    
    #[test] // test case 2: calculating neighbors
    pub fn test_neighbors() {
        assert_eq!(degree::neighbors(131), [676, 566, 891, 747]);
    }


}

 

  