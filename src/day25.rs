use std::{
    fs::File,
    process::{Command, Stdio},
};

use crate::util::file_lines;
use itertools::Itertools;
use petgraph::{
    algo::connected_components,
    dot::Dot,
    graph::UnGraph,
    prelude::NodeIndex,
    visit::{EdgeRef, NodeRef},
};
use std::io::Write;

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

    graph_to_svg(&graph, "day25_graph.svg");

    // Sanity check that we parsed the graph correctly
    assert_eq!(1, connected_components(&graph));
    let node_count = graph.node_count();

    // STOER, M., & WAGNER, F. (1997). A Simple Min-Cut Algorithm. Journal of the ACM, 44(4), 585-591.
    fn minimum_cut_phase(graph: &mut UnGraph<String, u32>, start_node: NodeIndex) -> (String, u32) {
        let mut subgroup = vec![start_node];
        while subgroup.len() != graph.node_count() {
            let nodes_not_in_subgroup =
                graph.node_indices().filter(|node| !subgroup.contains(node));

            let most_tightly_connected_node = nodes_not_in_subgroup
                .max_by_key(|node| {
                    let edges_to_nodes_in_subgroup = graph.edges(*node).filter(|edge| {
                        subgroup.contains(&edge.target()) || subgroup.contains(&edge.source())
                    });

                    // Sum of all weights of edges to nodes in subgroup
                    edges_to_nodes_in_subgroup
                        .map(|edge| edge.weight())
                        .sum::<u32>()
                })
                .unwrap();

            subgroup.push(most_tightly_connected_node);
        }

        // Merge last two nodes
        // here: Merge node b into node a
        let (merge_a, merge_b) = (subgroup[subgroup.len() - 2], subgroup[subgroup.len() - 1]);
        // Remember cut of the phase
        let cut_of_the_phase = (
            graph.node_weight(merge_b).unwrap().clone(),
            graph.edges(merge_b).map(|edge| edge.weight()).sum(),
        );
        let merge_b_name = graph.node_weight(merge_b).unwrap().clone();

        // Gather information for updating node a to make borrow checker happy
        let edges_b = graph
            .edges(merge_b)
            // Ignore edges between nodes a and b
            .filter(|edge| edge.target() != merge_a && edge.source() != merge_a)
            // Gather other end of edge (might be source or target) and weight
            .map(|edge| {
                (
                    if edge.target() == merge_b {
                        edge.source()
                    } else {
                        edge.target()
                    },
                    *edge.weight(),
                )
            })
            .collect_vec();
        // Update node a
        for edge in edges_b {
            let edge_from_a = graph.find_edge(merge_a, edge.0);
            match edge_from_a {
                None => {
                    graph.add_edge(merge_a, edge.0, edge.1);
                }
                Some(edge_from_a) => {
                    graph[edge_from_a] += edge.1;
                }
            };
        }

        //let cut_of_the_phase: u32 = graph.edges(merge_a).map(|edge| edge.weight()).sum();
        // Update name of node a
        graph[merge_a] += ",";
        graph[merge_a] += &merge_b_name;

        // Remove node b
        graph.remove_node(merge_b);

        cut_of_the_phase
    }

    fn minimum_cut(graph: &mut UnGraph<String, u32>) -> (String, u32) {
        let start_node = graph.node_indices().next().unwrap();
        let mut min_cut_of_the_phase: Option<(String, u32)> = None;
        let mut i = 1;
        while graph.node_count() > 1 {
            println!("{}", graph.node_count());
            let cut_of_the_phase = minimum_cut_phase(graph, start_node);
            //println!("{:?}", cut_of_the_phase);
            // TODO better pattern for this
            if let Some(min_cut_of_the_phase_v) = &min_cut_of_the_phase {
                if cut_of_the_phase.1 < min_cut_of_the_phase_v.1 {
                    min_cut_of_the_phase = Some(cut_of_the_phase);
                }
            } else {
                min_cut_of_the_phase = Some(cut_of_the_phase);
            }
            //graph_to_svg(graph, &format!("day25_graph{}.svg", i));
            i += 1;
        }
        min_cut_of_the_phase.unwrap()
    }

    let min_cut = minimum_cut(&mut graph);
    println!("{:?}", min_cut);
    assert_eq!(3, min_cut.1);
    let node_size_a = min_cut.0.split(',').count();
    let node_size_b = node_count - node_size_a;
    println!("{} / {}", node_size_a, node_size_b);
    println!("{:?}", node_size_a * node_size_b);
}

pub fn part2() {
    let lines = file_lines("inp25_1.txt");
}
