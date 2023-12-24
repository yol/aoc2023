// TODO remove unused code
// TODO make part1 work again
// TODO optimize part2 performance :-)

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::hash::BuildHasherDefault;
use std::hash::Hash;

use num::Zero;
use rustc_hash::FxHasher;

use super::util::{build_grid, file_lines, Direction, Position};
use grid::Grid;
use indexmap::IndexMap;

use indexmap::map::Entry::{Occupied, Vacant};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Path,
    Forest,
    SlopeTo(Direction),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct SearchState {
    pos: Position,
    dir: Option<Direction>,
    covered: BTreeSet<Position>,
}

fn advance_to_junction(
    grid: &Grid<Tile>,
    start_pos: Position,
    start_dir: Direction,
) -> Option<(
    Position,
    Direction, /* last direction that was walked in */
    usize,
)> {
    let mut dir = start_dir;
    let mut pos = start_pos.advance_in_grid(dir, grid)?;
    if grid[pos.as_grid_pos()] == Tile::Forest {
        return None;
    }
    let mut len = 1;
    loop {
        let mut new_positions = Direction::all().filter_map(|new_dir| {
            if new_dir == dir.opposite() {
                // Don't go back
                return None;
            }
            let new_pos = pos.advance_in_grid(new_dir, grid)?;
            if grid[new_pos.as_grid_pos()] == Tile::Forest {
                return None;
            }
            Some((new_dir, new_pos))
        });
        let next = new_positions.next();
        if next.is_none() || new_positions.next().is_some() {
            // Hit the end or more possible directions
            return Some((pos, dir, len));
        }

        len += 1;
        (dir, pos) = next.unwrap();
    }
}

impl SearchState {
    fn advance_in_grid_with_dir(
        &self,
        grid: &Grid<Tile>,
        dir: Direction,
    ) -> Option<(SearchState, usize)> {
        //let new_pos = self.pos.advance_in_grid(dir, grid)?;
        let (new_pos, _, path_len) = advance_to_junction(grid, self.pos, dir)?;
        //println!("{:?} {:?} -> {:?}", self.pos, dir, new_pos);
        if self.covered.contains(&new_pos) {
            return None;
        }
        /*let can_go = match grid[new_pos.as_grid_pos()] {
            Tile::Forest => false,
            Tile::Path => true,
            Tile::SlopeTo(slope_dir) => slope_dir == dir,
        };*/
        let mut new_covered = self.covered.clone();
        new_covered.insert(new_pos);
        //if can_go {
        Some((
            SearchState {
                pos: new_pos,
                dir: Some(dir),
                covered: new_covered,
            },
            path_len,
        ))
        /* } else {
            None
        }*/
    }
}

type FxIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;

// Copied from pathfinding's bfs.rs

struct SmallestHolder<K> {
    cost: K,
    index: usize,
}

impl<K: PartialEq> PartialEq for SmallestHolder<K> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<K: PartialEq> Eq for SmallestHolder<K> {}

impl<K: Ord> PartialOrd for SmallestHolder<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord> Ord for SmallestHolder<K> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn run_dijkstra<N, C, FN, IN, FS>(
    start: &N,
    successors: &mut FN,
    stop: &mut FS,
) -> (FxIndexMap<N, (usize, C)>, Option<usize>)
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    let mut to_see = BinaryHeap::new();
    to_see.push(SmallestHolder {
        cost: Zero::zero(),
        index: 0,
    });
    let mut parents: FxIndexMap<N, (usize, C)> = FxIndexMap::default();
    parents.insert(start.clone(), (usize::max_value(), Zero::zero()));
    let mut target_reached = None;
    while let Some(SmallestHolder { cost, index }) = to_see.pop() {
        let successors = {
            let (node, _) = parents.get_index(index).unwrap();
            if stop(node) {
                target_reached = Some(index);
                break;
            }
            successors(node)
        };
        for (successor, move_cost) in successors {
            let new_cost = cost + move_cost;
            let n;
            match parents.entry(successor) {
                Vacant(e) => {
                    n = e.index();
                    e.insert((index, new_cost));
                }
                Occupied(mut e) => {
                    if e.get().1 < new_cost {
                        n = e.index();
                        e.insert((index, new_cost));
                    } else {
                        continue;
                    }
                }
            }

            to_see.push(SmallestHolder {
                cost: new_cost,
                index: n,
            });
        }
    }
    (parents, target_reached)
}

pub fn dijkstra<N, C, FN, IN, FS>(
    start: &N,
    mut successors: FN,
    mut success: FS,
) -> (FxIndexMap<N, (usize, C)>, Option<usize>)
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    run_dijkstra(start, &mut successors, &mut success)
}

pub fn part1() {
    let lines = file_lines("inp23_2.txt");

    let grid = build_grid(&lines, |c| match c {
        '.' => Tile::Path,
        '#' => Tile::Forest,
        '^' => Tile::SlopeTo(Direction::N),
        '>' => Tile::SlopeTo(Direction::E),
        'v' => Tile::SlopeTo(Direction::S),
        '<' => Tile::SlopeTo(Direction::W),
        _ => panic!(),
    });

    let start_pos = Position {
        x: grid.iter_row(0).position(|&t| t == Tile::Path).unwrap() as isize,
        y: 0,
    };
    let res = dijkstra(
        &SearchState {
            pos: start_pos,
            dir: None,
            covered: BTreeSet::new(),
        },
        |s| {
            Direction::all()
                .filter_map(|new_dir| {
                    if s.dir.is_some() && new_dir == s.dir.unwrap().opposite() {
                        // Never go back
                        return None;
                    }
                    s.advance_in_grid_with_dir(&grid, new_dir)
                })
                .collect_vec()
        },
        |_| false, //|s| s.pos.y == (grid.rows() - 1) as isize,
    );
    let end_state = res
        .0
        .iter()
        .find(|e| e.0.pos.y == (grid.rows() - 1) as isize)
        .unwrap();
    let cost = end_state.1 .1;
    println!("{}", cost);
}

pub fn part2() {
    let lines = file_lines("inp23_2.txt");

    let grid = build_grid(&lines, |c| match c {
        '.' => Tile::Path,
        '#' => Tile::Forest,
        '^' | '>' | 'v' | '<' => Tile::Path,
        _ => panic!(),
    });
    let start_pos = Position {
        x: grid.iter_row(0).position(|&t| t == Tile::Path).unwrap() as isize,
        y: 0,
    };

    struct NodeLink {
        destination: usize,
        distance: usize,
    }

    impl Debug for NodeLink {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "(to {} dist {})", self.destination, self.distance)
        }
    }

    #[derive(Debug)]
    struct Node {
        pos: Position,
        next_nodes: Vec<NodeLink>,
    }
    let mut nodes: Vec<Node> = vec![Node {
        pos: start_pos,
        next_nodes: Vec::new(),
    }];

    struct QueueEntry {
        node: usize,
        dir: Direction,
    }

    let mut q = VecDeque::from([QueueEntry {
        node: 0,
        dir: Direction::S,
    }]);
    while let Some(entry) = q.pop_back() {
        let node = &nodes[entry.node];
        let next = advance_to_junction(&grid, node.pos, entry.dir);
        match next {
            None => {}
            Some((new_pos, last_dir, path_len)) => {
                let this_node_idx = match nodes.iter().position(|n| n.pos == new_pos) {
                    None => {
                        nodes.push(Node {
                            pos: new_pos,
                            next_nodes: Vec::new(),
                        });
                        let new_node_idx = nodes.len() - 1;

                        // Node newly created -> investigate all directions at junction (except where we came from)
                        for dir in Direction::all() {
                            if dir == last_dir.opposite() {
                                continue;
                            }
                            q.push_back(QueueEntry {
                                node: new_node_idx,
                                dir,
                            });
                        }

                        new_node_idx
                    }
                    Some(x) => x,
                };
                // Add both directions
                let prev_node = &mut nodes[entry.node];
                prev_node.next_nodes.push(NodeLink {
                    destination: this_node_idx,
                    distance: path_len,
                });
                let this_node = &mut nodes[this_node_idx];
                this_node.next_nodes.push(NodeLink {
                    destination: entry.node,
                    distance: path_len,
                });
            }
        };
    }

    println!("{:?}", nodes);

    let start_node = 0;
    let end_node = nodes
        .iter()
        .position(|n| n.pos.y as usize == grid.rows() - 1)
        .unwrap();

    #[derive(PartialEq, Eq, Hash, Clone)]
    struct SearchState2 {
        node: usize,
        visited: BTreeSet<usize>,
    }

    let res = dijkstra(
        &SearchState2 {
            node: start_node,
            visited: BTreeSet::new(),
        },
        |s| {
            let node = &nodes[s.node];
            node.next_nodes
                .iter()
                .filter_map(|link| {
                    if s.visited.contains(&link.destination) {
                        None
                    } else {
                        let mut new_visited = s.visited.clone();
                        new_visited.insert(link.destination);
                        Some((
                            SearchState2 {
                                node: link.destination,
                                visited: new_visited,
                            },
                            link.distance,
                        ))
                    }
                })
                .collect_vec()
        },
        |_| false,
    );

    let cost = res
        .0
        .iter()
        .filter(|e| e.0.node == end_node)
        .map(|s| s.1 .1)
        .max()
        .unwrap();
    println!("{}", cost);
}
