use std::collections::{HashMap, HashSet, VecDeque};

use petgraph::algo::toposort;
use petgraph::graphmap::DiGraphMap;
use petgraph::Direction;

use crate::config::registry::Registry;
use crate::tasks::error::Error;

pub type SignalIDs = Vec<String>;
pub type SourceTasks = HashMap<String, HashSet<String>>;

pub fn get_batched_tasks(registry: &Registry) -> Result<Vec<(SignalIDs, SourceTasks)>, Error> {
    let graph = build_graph(registry);
    let batches = build_batches(&graph, registry)?;
    Ok(batches)
}

fn build_graph(registry: &Registry) -> DiGraphMap<&str, ()> {
    let mut graph = DiGraphMap::<&str, ()>::new();

    for (k, v) in registry.iter() {
        if !graph.contains_node(k) {
            graph.add_node(k);
        }

        if let Some(pre_req) = &v.prerequisites {
            for id in pre_req {
                if id != k && !graph.contains_edge(k, id) {
                    if !graph.contains_node(id) {
                        graph.add_node(id);
                    }
                    graph.add_edge(id, k, ());
                }
            }
        }
    }

    graph
}

fn build_batches<'a>(
    graph: &'a DiGraphMap<&'a str, ()>,
    registry: &Registry,
) -> Result<Vec<(SignalIDs, SourceTasks)>, Error> {
    let sorted_nodes = toposort(&graph, None)?;
    let roots: Vec<&str> = sorted_nodes
        .iter()
        .filter(|n| {
            graph
                .neighbors_directed(n, Direction::Incoming)
                .next()
                .is_none()
        })
        .cloned()
        .collect();

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
                    if let Some(sources) = &signal.sources {
                        for source in sources {
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
            }
            tasks
        })
        .collect::<Vec<HashMap<String, HashSet<String>>>>();

    Ok(batches.into_iter().zip(tasks).collect())
}

fn bfs_with_depth<'a>(
    graph: &'a DiGraphMap<&'a str, ()>,
    start_roots: &[&str],
) -> (HashMap<String, usize>, usize) {
    let mut depths = HashMap::new();
    let mut max_depth = 0;

    for root in start_roots {
        let mut queue = VecDeque::new();

        queue.push_back(root.to_string());
        depths.insert(root.to_string(), 0);

        while let Some(node) = queue.pop_front() {
            let depth = depths[&node.to_string()];
            for neighbor in graph.neighbors_directed(node.as_str(), Direction::Outgoing) {
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
