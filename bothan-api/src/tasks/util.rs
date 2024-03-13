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
                        let uid = format!("{}{}", source_id, id);
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

    // Iterate over the roots and perform a BFS
    for root in start_roots {
        let mut queue = VecDeque::new();

        queue.push_back(root.to_string());
        depths.insert(root.to_string(), 0);

        // Perform BFS
        while let Some(node) = queue.pop_front() {
            let depth = depths[&node.to_string()];
            for neighbor in graph.neighbors_directed(&node, Direction::Outgoing) {
                if !depths.contains_key(&neighbor.to_string()) {
                    queue.push_back(neighbor.to_string());
                    depths.insert(neighbor.to_string(), depth + 1);
                    if depth + 1 > max_depth {
                        max_depth = depth + 1;
                    }
                } else if let Some(current_depth) = depths.get(&neighbor.to_string()) {
                    if depth + 1 < *current_depth {
                        queue.push_back(neighbor.to_string());
                        depths.insert(neighbor.to_string(), depth + 1);
                        if depth + 1 > max_depth {
                            max_depth = depth + 1;
                        }
                    }
                }
            }
        }
    }

    // Return the depths and the maximum depth
    (depths, max_depth)
}
