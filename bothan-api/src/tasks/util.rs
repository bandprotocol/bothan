use std::collections::{HashMap, HashSet, VecDeque};

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
            graph.add_edge(pid, id, ());
        }
    }

    Ok(graph)
}

// Builds a batch of tasks to be executed
fn build_batches(
    graph: &DiGraphMap<&String, ()>,
    registry: &Registry,
) -> Result<Vec<(SignalIDs, SourceTasks)>, Error> {
    // Builds the sequential order of batches which contains the signal ids to be executed
    let batches = batching_toposort(graph)?;

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

fn batching_toposort(graph: &DiGraphMap<&String, ()>) -> Result<Vec<Vec<String>>, Error> {
    let mut in_degree_counts = HashMap::new();
    let mut result = Vec::new();
    let mut roots = VecDeque::new();

    graph.nodes().for_each(|n| {
        let in_degree_count = graph.neighbors_directed(n, Direction::Incoming).count();
        in_degree_counts.insert(n.clone(), in_degree_count);
        if graph.neighbors_directed(n, Direction::Incoming).count() == 0 {
            roots.push_back(n.clone());
        }
    });

    while !roots.is_empty() {
        result.push(Vec::from(roots.clone()));

        let mut new_roots = VecDeque::new();
        roots.iter().for_each(|root| {
            graph
                .neighbors_directed(root, Direction::Outgoing)
                .for_each(|dep| {
                    // Unwrap here as map is must contain this value
                    // If not, function is not working as expected
                    let count = in_degree_counts.get_mut(dep).unwrap();
                    *count -= 1;
                    if *count == 0 {
                        new_roots.push_back(dep.clone());
                    }
                })
        });

        roots = new_roots;
    }

    if in_degree_counts.values().any(|&v| v > 0) {
        return Err(Error::CycleDetected());
    }

    Ok(result)
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
    fn test_batching_toposort() {
        let nodes = vec![
            ("A".to_string(), "B".to_string()),
            ("A".to_string(), "C".to_string()),
            ("C".to_string(), "D".to_string()),
            ("C".to_string(), "E".to_string()),
            ("E".to_string(), "F".to_string()),
        ];
        let graph = mock_graph(&nodes);

        let batches = batching_toposort(&graph);
        let expected = vec![
            vec!["A".to_string()],
            vec!["B".to_string(), "C".to_string()],
            vec!["D".to_string(), "E".to_string()],
            vec!["F".to_string()],
        ];
        // Assert that the returned depths match the expected values
        assert_eq!(batches.unwrap(), expected);
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

        let batches = batching_toposort(&graph);
        let expected = vec![
            vec!["F".to_string(), "D".to_string(), "B".to_string()],
            vec!["E".to_string()],
            vec!["C".to_string()],
            vec!["A".to_string()],
        ];
        assert_eq!(batches.unwrap(), expected);
    }

    #[test]
    fn test_bfs_with_depth_with_isolated_node_and_multiple_roots() {
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

        let batches = batching_toposort(&graph);
        let expected = vec![
            vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "G".to_string(),
            ],
            vec!["D".to_string(), "E".to_string()],
            vec!["F".to_string()],
        ];
        assert_eq!(batches.unwrap(), expected);
    }
}
