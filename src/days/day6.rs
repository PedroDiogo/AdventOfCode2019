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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_case_1() {
        let tree = vec!["COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L"];
        let tree = get_orbits_tree(&tree);
        assert_eq!(42, get_total_number_of_orbits(&tree))
    }
}