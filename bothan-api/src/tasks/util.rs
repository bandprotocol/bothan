use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};

use petgraph::algo::toposort;
use petgraph::graphmap::DiGraphMap;
use petgraph::Direction;

use crate::registry::{Registry, Signal};
use crate::tasks::error::Error;

pub type SignalIDs = Vec<String>;
pub type SignalMap = HashMap<String, Signal>;
pub type SourceTasks = HashMap<String, HashSet<String>>;

// Takes a registry and returns the sequential order of tasks to be executed
pub fn get_batched_tasks(registry: &Registry) -> Result<Vec<(SignalMap, SourceTasks)>, Error> {
    let graph = build_graph(registry)?;
    let batches = build_batches(&graph, registry)?;
    let batched_tasks = batches
        .into_iter()
        .map(|(signal_ids, source_tasks)| {
            let signals = signal_ids
                .iter()
                .map(|id| {
                    // Unwrapped here since Registry is expected to contain ID
                    // generated from batches
                    let signal = registry.get(id).unwrap().clone();
                    (id.clone(), signal)
                })
                .collect();
            (signals, source_tasks)
        })
        .collect::<Vec<(SignalMap, SourceTasks)>>();
    Ok(batched_tasks)
}

// Builds a directed graph from the registry
fn build_graph(registry: &Registry) -> Result<DiGraphMap<&String, ()>, Error> {
    let mut graph = DiGraphMap::<&String, ()>::new();

    for (id, signal) in registry.iter() {
        if !graph.contains_node(id) {
            graph.add_node(id);
        }

        for pid in &signal.prerequisites {
            if pid == id {
                return Err(Error::CycleDetected());
            }
            if !graph.contains_edge(id, pid) {
                if !graph.contains_node(pid) {
                    graph.add_node(pid);
                }
                graph.add_edge(pid, id, ());
            }
        }
    }

    Ok(graph)
}

// Builds a batch of tasks to be executed
fn build_batches(
    graph: &DiGraphMap<&String, ()>,
    registry: &Registry,
) -> Result<Vec<(SignalIDs, SourceTasks)>, Error> {
    let sorted_nodes = toposort(&graph, None)?;
    let roots = sorted_nodes
        .iter()
        .filter(|n| {
            graph
                .neighbors_directed(n, Direction::Incoming)
                .next()
                .is_none()
        })
        .cloned()
        .collect::<Vec<&String>>();

    let (depths, max_depth) = bfs_with_depth(graph, &roots);

    // Builds the sequential order of batches which contains the signal ids to be executed
    let mut batches = vec![Vec::new(); max_depth + 1];
    for (signal_id, depth) in depths.into_iter() {
        batches[depth].push(signal_id);
    }

    // Builds the source tasks for each batch
    let mut seen = HashSet::new();
    let source_tasks = batches
        .iter()
        .map(|batch| {
            let mut tasks = HashMap::new();
            for signal_id in batch {
                if let Some(signal) = registry.get(signal_id) {
                    for source in &signal.sources {
                        let source_id = source.source_id.clone();
                        let id = source.id.clone();
                        let uid = format!("{}-{}", source_id, id);
                        if !seen.contains(&uid) {
                            let entry = tasks.entry(source_id).or_insert(HashSet::new());
                            entry.insert(id);
                            seen.insert(uid);
                        }
                    }
                }
            }
            tasks
        })
        .collect::<Vec<HashMap<String, HashSet<String>>>>();

    Ok(batches.into_iter().zip(source_tasks).collect())
}

// Performs a breadth-first search on the graph, returning the depths of each node and the maximum depth
fn bfs_with_depth(
    graph: &DiGraphMap<&String, ()>,
    start_roots: &[&String],
) -> (HashMap<String, usize>, usize) {
    let mut depths = HashMap::new();
    let mut max_depth = 0;

    // Initialize the queue with the source nodes and their depths
    for root in start_roots {
        depths.insert(root.to_string(), 0);
    }

    let mut queue: VecDeque<_> = start_roots.iter().copied().collect::<VecDeque<&String>>();

    // Perform Multi-Source BFS
    while let Some(node) = queue.pop_front() {
        for neighbor in graph.neighbors_directed(node, Direction::Outgoing) {
            let depth = depths[&node.to_string()];
            match depths.entry(neighbor.clone()) {
                Entry::Occupied(o) => {
                    // If node depth is already set, check if the current depth is less than the
                    // new depth. If the new depth is larger, override the current depth with the
                    // new depth.
                    let new_depth = depth + 1;
                    if new_depth > *o.get() {
                        queue.push_back(neighbor);
                        depths.insert(neighbor.to_string(), new_depth);
                    }
                    if new_depth > max_depth {
                        max_depth = new_depth;
                    }
                }
                Entry::Vacant(v) => {
                    queue.push_back(neighbor);
                    v.insert(depth + 1);
                    if depth + 1 > max_depth {
                        max_depth = depth + 1;
                    }
                }
            }
        }
    }

    // Return the depths and the maximum depth
    (depths, max_depth)
}

#[cfg(test)]
mod tests {
    use petgraph::graphmap::DiGraphMap;

    use super::*;

    fn mock_graph(node_pairs: &Vec<(String, String)>) -> DiGraphMap<&String, ()> {
        let mut graph = DiGraphMap::<&String, ()>::new();
        for (n1, n2) in node_pairs {
            graph.add_edge(n1, n2, ());
        }
        graph
    }

    #[test]
    fn test_bfs_with_depth() {
        // Create a new graph
        let nodes = vec![
            ("A".to_string(), "B".to_string()),
            ("A".to_string(), "C".to_string()),
            ("C".to_string(), "D".to_string()),
            ("C".to_string(), "E".to_string()),
            ("E".to_string(), "F".to_string()),
        ];
        let graph = mock_graph(&nodes);

        // Call bfs_with_depth with the graph and a known set of root nodes
        let roots = vec![&nodes[0].0];
        let (depths, max_depth) = bfs_with_depth(&graph, roots.as_slice());

        // Assert that the returned depths match the expected values
        assert_eq!(depths[&"A".to_string()], 0);
        assert_eq!(depths[&"B".to_string()], 1);
        assert_eq!(depths[&"C".to_string()], 1);
        assert_eq!(depths[&"D".to_string()], 2);
        assert_eq!(depths[&"E".to_string()], 2);
        assert_eq!(depths[&"F".to_string()], 3);

        // Assert that the returned maximum depth matches the expected value
        assert_eq!(max_depth, 3);
    }

    #[test]
    fn test_bfs_with_depth_with_multiple_roots() {
        // Create a new graph
        let nodes = vec![
            ("F".to_string(), "E".to_string()),
            ("E".to_string(), "C".to_string()),
            ("D".to_string(), "C".to_string()),
            ("C".to_string(), "A".to_string()),
            ("B".to_string(), "A".to_string()),
        ];
        let graph = mock_graph(&nodes);

        // Call bfs_with_depth with the graph and a known set of root nodes
        let roots = vec![&nodes[0].0, &nodes[2].0, &nodes[4].0];
        let (depths, max_depth) = bfs_with_depth(&graph, roots.as_slice());

        // Assert that the returned depths match the expected values
        assert_eq!(depths[&"F".to_string()], 0);
        assert_eq!(depths[&"D".to_string()], 0);
        assert_eq!(depths[&"B".to_string()], 0);
        assert_eq!(depths[&"E".to_string()], 1);
        assert_eq!(depths[&"C".to_string()], 2);
        assert_eq!(depths[&"A".to_string()], 3);

        // Assert that the returned maximum depth matches the expected value
        assert_eq!(max_depth, 3);
    }
    #[test]
    fn test_bfs_with_depth_with_isolated_node_and_multiple_roots() {
        // Create a new graph
        let nodes = vec![
            ("A".to_string(), "D".to_string()),
            ("B".to_string(), "D".to_string()),
            ("B".to_string(), "F".to_string()),
            ("C".to_string(), "E".to_string()),
            ("E".to_string(), "F".to_string()),
        ];
        let mut graph = mock_graph(&nodes);
        let sole_node = "G".to_string();
        graph.add_node(&sole_node);

        // Call bfs_with_depth with the graph and a known set of root nodes
        let roots = vec![&nodes[0].0, &nodes[1].0, &nodes[3].0, &sole_node];
        let (depths, max_depth) = bfs_with_depth(&graph, roots.as_slice());

        // Assert that the returned depths match the expected values
        assert_eq!(depths[&"A".to_string()], 0);
        assert_eq!(depths[&"B".to_string()], 0);
        assert_eq!(depths[&"C".to_string()], 0);
        assert_eq!(depths[&"G".to_string()], 0);
        assert_eq!(depths[&"D".to_string()], 1);
        assert_eq!(depths[&"E".to_string()], 1);
        assert_eq!(depths[&"F".to_string()], 2);

        // Assert that the returned maximum depth matches the expected value
        assert_eq!(max_depth, 2);
    }
}
