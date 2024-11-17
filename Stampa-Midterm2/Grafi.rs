use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::hash::Hash;
use std::rc::Rc;

// Type alias for a reference-counted Node
type NodeRef<T> = Rc<Node<T>>;

// Node struct
#[derive(Clone)]
struct Node<T: Eq + PartialEq + Hash> {
    value: T,
    adjacents: Vec<NodeRef<T>>,
}

impl<T: Eq + PartialEq + Hash + Debug> Node<T> {
    // Create a new Node
    fn new(value: T, adjacents: Vec<NodeRef<T>>) -> Self {
        Self { value, adjacents }
    }

    // Get a reference to the node's value
    fn get_value(&self) -> &T {
        &self.value
    }
}

// Implement Debug for Node
impl<T: Eq + PartialEq + Hash + Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let adj_values: Vec<String> = self
            .adjacents
            .iter()
            .map(|n| format!("{:?}", n.get_value()))
            .collect();
        write!(
            f,
            "[value: {:?}, adjacents: \"[{}]\"]",
            self.value,
            adj_values.join(", ")
        )
    }
}

// Graph struct
struct Graph<T: Eq + PartialEq + Hash> {
    nodes: Vec<NodeRef<T>>,
}

impl<T: Eq + PartialEq + Hash + Debug + Clone> Graph<T> {
    // Create a new Graph from a vector of nodes
    fn new(nodes: Vec<NodeRef<T>>) -> Self {
        Self { nodes }
    }

    // Depth-first search
    fn dfs(&self, start: NodeRef<T>) -> Vec<NodeRef<T>> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        self.dfs_recursive(&start, &mut visited, &mut result);
        result
    }

    // Recursive helper function for DFS
    fn dfs_recursive(
        &self,
        node: &NodeRef<T>,
        visited: &mut HashSet<T>,
        result: &mut Vec<NodeRef<T>>,
    ) where
        T: Clone,
    {
        if visited.contains(node.get_value()) {
            return;
        }

        visited.insert(node.get_value().clone());
        result.push(node.clone());

        for adj in &node.adjacents {
            self.dfs_recursive(adj, visited, result);
        }
    }
}

fn main() {
    let n1 = Rc::new(Node::new(1, vec![]));
    let n2 = Rc::new(Node::new(2, vec![n1.clone()]));
    let n3 = Rc::new(Node::new(3, vec![]));
    let n4 = Rc::new(Node::new(4, vec![n1.clone(), n3.clone()]));
    let n5 = Rc::new(Node::new(5, vec![n2.clone(), n4.clone()]));
    let n6 = Rc::new(Node::new(6, vec![n5.clone(), n4.clone()]));
    let n7 = Rc::new(Node::new(7, vec![n2.clone(), n4.clone()]));

    let graph = Graph::new(vec![
        n1.clone(),
        n2.clone(),
        n3.clone(),
        n4.clone(),
        n5.clone(),
        n6.clone(),
        n7.clone(),
    ]);

    let mut paths: Vec<Vec<NodeRef<i32>>> = vec![];
    for n in graph.nodes.iter() {
        paths.push(graph.dfs(n.clone()))
    }

    paths.iter().for_each(|path| {
        println!("{:?}", path);
    });
}
