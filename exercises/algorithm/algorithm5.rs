/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        // 用于存储已访问的节点
        let mut visited = vec![false; self.adj.len()];
        // 用于存储待访问的节点队列
        let mut queue = VecDeque::new();
        // 用于记录节点访问顺序的结果数组
        let mut visit_order = vec![];
        
        // 将起始节点标记为已访问
        visited[start] = true;
        // 将起始节点加入队列
        queue.push_back(start);
        // 将起始节点添加到访问顺序列表中
        visit_order.push(start);
        
        // BFS 的主循环
        while let Some(current) = queue.pop_front() {
            // 遍历当前节点的所有邻接节点
            for &neighbor in &self.adj[current] {
                // 如果邻接节点尚未访问
                if !visited[neighbor] {
                    // 将邻接节点标记为已访问
                    visited[neighbor] = true;
                    // 将邻接节点加入队列以便后续处理
                    queue.push_back(neighbor);
                    // 将邻接节点添加到访问顺序列表中
                    visit_order.push(neighbor);
                }
            }
        }
        
        visit_order
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

