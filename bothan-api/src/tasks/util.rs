use std::collections::{HashMap, HashSet, VecDeque};

use petgraph::algo::toposort;
use petgraph::graphmap::DiGraphMap;
use petgraph::Direction;

use crate::registry::{Registry, Signal};
use crate::tasks::error::Error;

pub type SignalIDs = Vec<String>;
pub type SignalMap = HashMap<String, Signal>;
pub type SourceTasks = HashMap<String, HashSet<String>>;

pub fn get_batched_tasks(registry: &Registry) -> Result<Vec<(SignalMap, SourceTasks)>, Error> {
    let graph = build_graph(registry);
    let batches = build_batches(&graph, registry)?;
    Ok(batches
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
        .collect::<Vec<(HashMap<String, Signal>, SourceTasks)>>())
}

fn build_graph(registry: &Registry) -> DiGraphMap<&String, ()> {
    let mut graph = DiGraphMap::<&String, ()>::new();

    for (k, v) in registry.iter() {
        if !graph.contains_node(k) {
            graph.add_node(k);
        }

        for id in &v.prerequisites {
            if id != k && !graph.contains_edge(k, id) {
                if !graph.contains_node(id) {
                    graph.add_node(id);
                }
                graph.add_edge(id, k, ());
            }
        }
    }

    graph
}

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
    let mut batches = vec![Vec::new(); max_depth + 1];
    for (k, v) in depths.into_iter() {
        batches[v].push(k);
    }

    let mut seen = HashSet::new();
    let tasks = batches
        .iter()
        .map(|batch| {
            let mut tasks = HashMap::new();
            for id in batch {
                if let Some(signal) = registry.get(id) {
                    for source in &signal.sources {
                        let source_id = source.source_id.clone();
                        let id = source.id.clone();
                        let tup: (String, String) = (source_id.clone(), id.clone());
                        if !seen.contains(&tup) {
                            let entry = tasks.entry(source_id).or_insert(HashSet::new());
                            entry.insert(id);
                            seen.insert(tup);
                        }
                    }
                }
            }
            tasks
        })
        .collect::<Vec<HashMap<String, HashSet<String>>>>();

    Ok(batches.into_iter().zip(tasks).collect())
}

fn bfs_with_depth(
    graph: &DiGraphMap<&String, ()>,
    start_roots: &[&String],
) -> (HashMap<String, usize>, usize) {
    let mut depths = HashMap::new();
    let mut max_depth = 0;

    for root in start_roots {
        let mut queue = VecDeque::new();

        queue.push_back(root.to_string());
        depths.insert(root.to_string(), 0);

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

    (depths, max_depth)
}
