extern crate advent;
use self::advent::*;

use std::collections::HashMap;

pub fn run() {
    let filename = "inputs/day6.txt";
    let inputs = read_inputs(filename);
    let inputs = split_lines_into_vec_str(&inputs);

    let orbits_tree = get_orbits_tree(&inputs);
    let part_one = get_total_number_of_orbits(&orbits_tree);

    println!("Part one: {}", part_one);

    let part_two = get_distance_between_you_and_san(&orbits_tree);

    println!("Part two: {}", part_two);
}

fn get_orbits_tree<'a>(inputs: &'a Vec<&str>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut all_nodes : HashMap<&str, Vec<&str>> = HashMap::new();

    for input in inputs.clone() {
        let mut input = input.split(")");
        let source = input.next().expect("Couldnt get source node");
        let dest = input.next().expect("Couldnt get dest node");

        let mut source_children : Vec<&str> = if let Some(source_children) = all_nodes.get(source) {
            source_children.clone()
        } else {
            Vec::new()
        };

        source_children.push(dest);
        all_nodes.insert(source, source_children);
    }

    all_nodes
}

fn get_total_number_of_orbits(orbits_tree: &HashMap<&str, Vec<&str>>) -> i32 {
    let root = "COM";
    let root_orbits = 0;
    let starting_queue : Vec<(&str, i32)> = orbits_tree.get(root).unwrap().into_iter().map(|child| (*child, root_orbits)).collect();

    let mut queue : Vec<(&str, i32)> = starting_queue;
    let mut total_orbits = root_orbits;

    while !queue.is_empty() {
        let (node, parent_orbits) = queue.remove(0);
        let node_orbits = parent_orbits + 1;
        total_orbits += node_orbits;

        let next : Vec<(&str, i32)> = orbits_tree.get(node).unwrap_or(&vec![]).into_iter().map(|child| (*child, node_orbits)).collect();
        queue.extend(next);
    }

    total_orbits
}

fn get_path_until_node<'a>(orbits_tree: &HashMap<&str, Vec<&'a str>>, node: &str) -> Vec<&'a str> {
    let root = "COM";
    let mut queue : Vec<(&str, Vec<&'a str>)> = vec![(root, vec![])];

    let mut path : Vec<&'a str> = Vec::new();
    while !queue.is_empty() {
        let (child_node, child_path) = queue.remove(0);

        if child_node == node {
            path = child_path;
            break;
        }

        for next_node in orbits_tree.get(child_node).unwrap_or(&vec![]) {
            let mut next_path : Vec<&'a str> = child_path.clone();
            next_path.push(next_node);
            queue.push((next_node, next_path));
        }
    }
    path
}

fn get_distance_between_you_and_san(orbits_tree: &HashMap<&str, Vec<&str>>) -> i32 {
    let path_to_you = get_path_until_node(orbits_tree, "YOU");
    let path_to_san = get_path_until_node(orbits_tree, "SAN");
    let min_len = std::cmp::min(path_to_you.len(), path_to_san.len());

    let mut distance = 0;
    for i in 0..min_len {
        if path_to_you[i] != path_to_san[i] {
            distance = path_to_you.len()-i-1 + path_to_san.len()-i-1;
            break;
        }
    }

    distance as i32
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_case_1() {
        let tree = vec!["COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L"];
        let tree = get_orbits_tree(&tree);
        assert_eq!(42, get_total_number_of_orbits(&tree))
    }

    #[test]
    fn test_get_path_until_node() {
        let tree = vec!["COM)B","B)C","C)D","D)E","E)F","B)G","G)H","D)I","E)J","J)K","K)L","K)YOU","I)SAN"];
        let expected_path_until_you = vec!["B", "C", "D", "E", "J", "K", "YOU"];
        let tree = get_orbits_tree(&tree);
        assert_eq!(expected_path_until_you, get_path_until_node(&tree, "YOU"))
    }

    #[test]
    fn test_case_2() {
        let tree = vec!["COM)B","B)C","C)D","D)E","E)F","B)G","G)H","D)I","E)J","J)K","K)L","K)YOU","I)SAN"];
        let tree = get_orbits_tree(&tree);

        assert_eq!(4, get_distance_between_you_and_san(&tree));
    }
}