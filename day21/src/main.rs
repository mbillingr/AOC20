use common::input::Input;
use common::itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = Input::from_file("data/day21-input.txt");

    /*let input = Input::from_str("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
    trh fvjkl sbzzf mxmxvkd (contains dairy)
    sqjhc fvjkl (contains soy)
    sqjhc mxmxvkd sbzzf (contains fish)");*/

    let data = input
        .iter_lines()
        .map(|line| {
            let (ingredients, contained_allergens) =
                line.split(" (contains ").collect_tuple().unwrap();

            let ingredients = ingredients.split_whitespace().collect_vec();
            let contained_allergens = contained_allergens[..contained_allergens.len() - 1]
                .split(", ")
                .collect_vec();
            (ingredients, contained_allergens)
        })
        //.inspect(|x| println!("{:?}", x))
        .collect_vec();

    let all_ingredients: HashSet<&str> = data.iter().flat_map(|(x, _)| x).copied().collect();
    println!("{:?}", all_ingredients);

    let all_allergens: HashSet<&str> = data.iter().flat_map(|(_, x)| x).copied().collect();
    println!("{:?}", all_allergens);

    let mut combinations: HashSet<(&str, &str)> = HashSet::new();
    for &ing in &all_ingredients {
        for &alg in &all_allergens {
            combinations.insert((ing, alg));
        }
    }

    for (ing, alg) in &data {
        for i in &all_ingredients {
            if !ing.contains(i) {
                for a in alg {
                    combinations.remove(&(*i, a));
                }
            }
        }
    }

    let allergenic: HashSet<&str> = combinations.iter().map(|(x, _)| *x).collect();

    let non_allergenic: HashSet<&str> = all_ingredients
        .iter()
        .filter(|&ing| !allergenic.contains(ing))
        .copied()
        .collect();

    let n = data
        .iter()
        .flat_map(|(x, _)| x)
        .filter(|&ing| non_allergenic.contains(ing))
        .count();
    println!("Part 1: {}", n);

    let mut all_allergens = all_allergens.into_iter().collect_vec();
    all_allergens.sort();
    for &alg in &all_allergens {
        print!("{}: ", alg);
        for &ing in &all_ingredients {
            if combinations.contains(&(ing, alg)) {
                print!(" {}", ing);
            }
        }
        println!();
    }

    // Manual solution
    // could be implemented by eliminating entries in combinations

    println!(
        "Part 2: {}",
        "vv,nlxsmb,rnbhjk,bvnkk,ttxvphb,qmkz,trmzkcfg,jpvz"
    )
}
