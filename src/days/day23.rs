use std::collections::{HashMap, HashSet};

pub fn run() -> String {
    let input = include_str!("../../inputs/day23.txt");
    solve(input)
}

fn solve(input: &str) -> String {
    let connections: Vec<Vec<&str>> = input.split_terminator("\n").map(|connection| connection.split("-").collect()).collect();
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    // build connection graph
    for connection in connections {
        let (node1, node2) = (connection[0], connection[1]);
        graph.entry(node1).or_insert_with(HashSet::new).insert(node2);
        graph.entry(node2).or_insert_with(HashSet::new).insert(node1);
    }

    let mut largest_clique = HashSet::new();

    for &node in graph.keys() {
        let mut clique = HashSet::new();
        clique.insert(node);
        find_clique(node, &graph, &mut clique);
        if clique.len() > largest_clique.len() {
            largest_clique = clique;
        }
    }

    let mut sorted_clique: Vec<&str> = largest_clique.into_iter().collect();
    sorted_clique.sort();
    sorted_clique.join(",")
}

fn find_clique<'a>(node: &'a str, graph: &'a HashMap<&'a str, HashSet<&'a str>>, clique: &mut HashSet<&'a str>) {
    if let Some(neighbors) = graph.get(node) {
        for &neighbor in neighbors {
            if clique.iter().all(|&n| graph.get(n).unwrap().contains(neighbor)) {
                clique.insert(neighbor);
            }
        }
    }
}
