use crate::util::file_lines;
use itertools::Itertools;
use petgraph::{
    algo::connected_components, dot::Dot, graph::UnGraph, prelude::NodeIndex, visit::EdgeRef,
};
use std::{
    collections::BTreeSet,
    fs::File,
    io::Write,
    process::{Command, Stdio},
};

fn graph_to_svg(graph: &UnGraph<String, u32>, filename: &str) {
    let mut dot = Command::new("dot")
        .arg("-Tsvg")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    dot.stdin
        .as_mut()
        .unwrap()
        .write_all(format!("{:?}", Dot::new(&graph)).as_bytes())
        .unwrap();

    let output = dot.wait_with_output().unwrap();
    let mut svgfile = File::create(filename).unwrap();
    svgfile.write_all(&output.stdout).unwrap();
}

pub fn part1() {
    let lines = file_lines("inp25_2.txt");
    let mut graph = UnGraph::<String, u32>::default();
    fn find_or_insert_node(graph: &mut UnGraph<String, u32>, name: &str) -> NodeIndex {
        match graph.node_indices().find(|i| graph[*i] == name) {
            Some(x) => x,
            None => graph.add_node(name.to_string()),
        }
    }
    for l in &lines {
        let (name, e_strs) = l.split_once(": ").unwrap();
        let n = find_or_insert_node(&mut graph, name);
        for e in e_strs.split(' ') {
            let e_index = find_or_insert_node(&mut graph, e);
            graph.add_edge(n, e_index, 1);
        }
    }

    // Sanity check that we parsed the graph correctly
    assert_eq!(1, connected_components(&graph));
    let node_count = graph.node_count();

    if node_count < 100 {
        graph_to_svg(&graph, "day25_graph.svg");
    }

    let min_cut = minimum_cut(&mut graph);
    println!("Minimum cut: {:?}", min_cut);
    // All edges have weight 1 and the solution requires to cut exactly 3 edges
    // -> weight of the minimum cut has to be 3.
    assert_eq!(3, min_cut.1);

    let nodes_a = min_cut.0.split(',');
    let node_count_a = nodes_a.count();
    let node_count_b = node_count - node_count_a;
    println!(
        "{} / {} -> {}",
        node_count_a,
        node_count_b,
        node_count_a * node_count_b
    );
}

// STOER, M., & WAGNER, F. (1997). A Simple Min-Cut Algorithm. Journal of the ACM, 44(4), 585-591.

fn minimum_cut_phase(graph: &mut UnGraph<String, u32>, start_node: NodeIndex) -> (String, u32) {
    let mut subgroup = BTreeSet::from_iter([start_node]);
    let mut not_subgroup = BTreeSet::from_iter(graph.node_indices().filter(|&n| n != start_node));

    // Loop will run at least once, setting s
    let mut s = NodeIndex::default();
    let mut t = start_node;

    while subgroup.len() != graph.node_count() {
        let most_tightly_connected_node =
            find_most_tightly_connected_node(graph, &subgroup, &not_subgroup);

        subgroup.insert(most_tightly_connected_node);
        not_subgroup.remove(&most_tightly_connected_node);

        s = t;
        t = most_tightly_connected_node;
    }

    // Remember cut of the phase
    let cut_of_the_phase = (
        graph.node_weight(t).unwrap().clone(),
        graph.edges(t).map(|edge| edge.weight()).sum(),
    );

    // Merge last two nodes
    // here: Merge node t into node s
    merge_nodes(graph, s, t);

    cut_of_the_phase
}

fn minimum_cut(graph: &mut UnGraph<String, u32>) -> (String, u32) {
    // Choose first node as initial node "a"
    let start_node = graph.node_indices().next().unwrap();
    let mut min_cut: Option<(String, u32)> = None;
    while graph.node_count() > 1 {
        println!("{}", graph.node_count());
        let cut_of_the_phase = minimum_cut_phase(graph, start_node);
        min_cut = match min_cut {
            // First round
            None => Some(cut_of_the_phase),
            // Better cut
            Some(prev_min_cut) if cut_of_the_phase.1 < prev_min_cut.1 => Some(cut_of_the_phase),
            // Worse or same weight cut
            Some(prev_min_cut) => Some(prev_min_cut),
        };
    }
    min_cut.unwrap()
}

fn merge_nodes(graph: &mut UnGraph<String, u32>, s: NodeIndex, t: NodeIndex) {
    let t_name = graph.node_weight(t).unwrap().clone();

    // Gather information for updating nodes to make borrow checker happy
    let edges_t = graph
        .edges(t)
        // Ignore edges between nodes s and t
        .filter(|edge| edge.target() != s && edge.source() != s)
        // Gather other end of edge (might be source or target) and weight
        .map(|edge| {
            (
                if edge.target() == t {
                    edge.source()
                } else {
                    edge.target()
                },
                *edge.weight(),
            )
        })
        .collect_vec();
    // Update node s
    for edge in edges_t {
        let edge_from_s = graph.find_edge(s, edge.0);
        match edge_from_s {
            None => {
                graph.add_edge(s, edge.0, edge.1);
            }
            Some(edge_from_s) => {
                graph[edge_from_s] += edge.1;
            }
        };
    }

    // Update name of node s
    graph[s] += ",";
    graph[s] += &t_name;

    // Remove node t
    graph.remove_node(t);
}

fn find_most_tightly_connected_node(
    graph: &mut UnGraph<String, u32>,
    subgroup: &BTreeSet<NodeIndex>,
    not_subgroup: &BTreeSet<NodeIndex>,
) -> NodeIndex {
    *not_subgroup
        .iter()
        .max_by_key(|&node| {
            let edges_to_nodes_in_subgroup = graph.edges(*node).filter(|edge| {
                (edge.target() != *node && subgroup.contains(&edge.target()))
                    || (edge.source() != *node && subgroup.contains(&edge.source()))
            });

            // Sum of all weights of edges to nodes in subgroup
            edges_to_nodes_in_subgroup
                .map(|edge| edge.weight())
                .sum::<u32>()
        })
        .unwrap()
}

pub fn part2() {
    // trololo
}
