use common::input::Input;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = Input::from_file("data/day07-input.txt");

    let mut backward_edges = HashMap::new();
    let mut forward_edges = HashMap::new();

    for line in input.iter_lines() {
        let mut outer_split = line.split(" bags contain ");
        let container_color = outer_split.next().unwrap();
        let contains = outer_split.next().unwrap();

        for content in contains.split(", ") {
            let mut spec = content.split_whitespace();
            match spec.next().unwrap() {
                "no" => {}
                num => {
                    let n: usize = num.parse().unwrap();
                    let c1 = spec.next().unwrap().to_string();
                    let c2 = spec.next().unwrap().to_string();
                    let inner_color = format!("{} {}", c1, c2);

                    backward_edges
                        .entry(inner_color.clone())
                        .or_insert(vec![])
                        .push((n, container_color.to_string()));

                    forward_edges
                        .entry(container_color.to_string())
                        .or_insert(vec![])
                        .push((n, inner_color));
                }
            }
        }
    }

    println!(
        "Part 1: {:?}",
        find_contains("shiny gold", &backward_edges).len()
    );

    println!(
        "Part 2: {:?}",
        find_total_inside("shiny gold", &forward_edges)
    );
}

fn find_contains<'a>(
    color: &str,
    edges: &'a HashMap<String, Vec<(usize, String)>>,
) -> HashSet<&'a str> {
    if !edges.contains_key(color) {
        return HashSet::new();
    }

    let mut result: HashSet<&str> = HashSet::new();
    for (_, container) in &edges[color] {
        result.insert(container);
        result.extend(find_contains(container, edges));
    }
    result
}

fn find_total_inside(color: &str, edges: &HashMap<String, Vec<(usize, String)>>) -> usize {
    if !edges.contains_key(color) {
        return 0;
    }

    let mut result = 0;
    for (n, container) in &edges[color] {
        result += n * (1 + find_total_inside(&container, edges));
    }
    result
}
