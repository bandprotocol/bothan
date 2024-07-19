use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};

use petgraph::graphmap::DiGraphMap;
use petgraph::Direction;

use crate::registry::Registry;
use crate::tasks::error::Error;
use crate::tasks::signal_task::SignalTask;
use crate::tasks::source_task::SourceTask;

pub type SourceMap = HashMap<String, HashSet<String>>;

/// Takes a registry and returns the sequential order of tasks to be executed
pub fn get_tasks(registry: &Registry) -> Result<(Vec<SignalTask>, Vec<SourceTask>), Error> {
    let graph = build_graph(registry)?;
    let source_tasks = get_source_tasks(registry)?;
    let sorted_signal_ids = batching_topo_sort(&graph)?;
    let signal_tasks = sorted_signal_ids
        .into_iter()
        .map(|id| registry.get(&id).map(|s| SignalTask::new(id, s.clone())))
        .collect::<Option<Vec<SignalTask>>>()
        .ok_or(Error::MissingNode)?;
    //
    // signal.sources.iter().all(|source| {
    //     source
    //         .routes
    //         .iter()
    //         .all(|route| signal.prerequisites.contains(&route.signal_id))
    // })

    Ok((signal_tasks, source_tasks))
}

// Builds a directed graph from the registry
fn build_graph(registry: &Registry) -> Result<DiGraphMap<&String, ()>, Error> {
    let mut graph = DiGraphMap::<&String, ()>::new();
    let mut queue = VecDeque::from_iter(registry.keys());
    let mut seen = HashSet::new();

    while let Some(signal_id) = queue.pop_front() {
        let signal = registry.get(signal_id).ok_or(Error::MissingNode)?;

        graph.add_node(signal_id);
        let prerequisites = signal
            .source_queries
            .iter()
            .flat_map(|s| s.routes.iter().map(|r| &r.signal_id))
            .collect::<Vec<&String>>();

        for pid in prerequisites {
            graph.add_edge(pid, signal_id, ());
            if !seen.contains(pid) {
                queue.push_back(pid);
                seen.insert(pid);
            }
        }
    }

    Ok(graph)
}

// Get all grouped source ids by source from a registry
fn get_source_tasks(registry: &Registry) -> Result<Vec<SourceTask>, Error> {
    let mut source_map: SourceMap = HashMap::new();

    for signal in registry.values() {
        for source in &signal.source_queries {
            match source_map.entry(source.source_id.clone()) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().insert(source.query_id.clone());
                }
                Entry::Vacant(entry) => {
                    let set = HashSet::from([source.query_id.clone()]);
                    entry.insert(set);
                }
            }
        }
    }

    let source_tasks = source_map
        .into_iter()
        .map(|(source, ids)| SourceTask::new(source, ids))
        .collect();

    Ok(source_tasks)
}

fn batching_topo_sort(graph: &DiGraphMap<&String, ()>) -> Result<Vec<String>, Error> {
    let mut in_degree_counts = HashMap::new();
    let mut result = Vec::with_capacity(graph.node_count());
    let mut roots = Vec::new();

    // Create a map of incoming edges for each node
    for node in graph.nodes() {
        let in_degree_count = graph.neighbors_directed(node, Direction::Incoming).count();
        in_degree_counts.insert(node.clone(), in_degree_count);
        if in_degree_count == 0 {
            roots.push(node.clone());
        }
    }

    // Perform Kahn's algorithm to find the topological order
    while !roots.is_empty() {
        result.append(&mut roots.clone());

        let mut new_roots = Vec::new();
        for root in roots.iter() {
            for dep in graph.neighbors_directed(root, Direction::Outgoing) {
                // Unwrap here as map is must contain this value
                // If not, function is not working as expected
                let count = in_degree_counts.get_mut(dep).unwrap();
                *count -= 1;
                if *count == 0 {
                    new_roots.push(dep.clone());
                }
            }
        }

        roots = new_roots;
    }

    if in_degree_counts.values().any(|&v| v > 0) {
        Err(Error::CycleDetected)
    } else {
        Ok(result)
    }
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
    fn test_batching_topo_sort() {
        let nodes = vec![
            ("A".to_string(), "B".to_string()),
            ("A".to_string(), "C".to_string()),
            ("C".to_string(), "D".to_string()),
            ("C".to_string(), "E".to_string()),
            ("E".to_string(), "F".to_string()),
        ];
        let graph = mock_graph(&nodes);

        let batches = batching_topo_sort(&graph);
        let expected = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
            "F".to_string(),
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

        let batches = batching_topo_sort(&graph);
        let expected = vec![
            "F".to_string(),
            "D".to_string(),
            "B".to_string(),
            "E".to_string(),
            "C".to_string(),
            "A".to_string(),
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

        let batches = batching_topo_sort(&graph);
        let expected = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "G".to_string(),
            "D".to_string(),
            "E".to_string(),
            "F".to_string(),
        ];
        assert_eq!(batches.unwrap(), expected);
    }
}
