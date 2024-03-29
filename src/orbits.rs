use std::collections::{HashMap, BTreeSet};
use std::borrow::Borrow;

use crate::input::load_input_lines_str;

pub fn day6_1() {
//    let input_lines = "COM)B
//B)C
//C)D
//D)E
//E)F
//B)G
//G)H
//D)I
//E)J
//J)K
//K)L".split_ascii_whitespace();
    let input_lines = load_input_lines_str("day6.txt");
    let mut direct_orbits: HashMap<String,BTreeSet<String>> = HashMap::new();
    for line in input_lines.iter() {
        let parts: Vec<String> = line.split(')').map(|s| s.to_owned()).collect();
        let key = parts[0].to_owned();
        let entry = direct_orbits.entry(key).or_insert(BTreeSet::new());
        entry.insert(parts[1].to_owned());
    }
    for (k, v) in direct_orbits.iter() {
        println!("{})){:?}", k, v);
    }
    let mut stack: Vec<(String,Vec<String>)> = vec![("COM".to_owned(), vec![])];
    let mut orbitcount = 0;
    loop {
        if let Some((key, parents)) = stack.pop() {
            if let Some(children) = direct_orbits.get(&key) {
                println!("Node {} with parents {:?}", &key, &parents);
                orbitcount += parents.len();
                let mut new_parents = parents.clone();
                new_parents.append(&mut vec![key]);
                for child in children.iter() {
                    stack.push((child.clone(), new_parents.clone()));
                }
            } else {
                orbitcount += parents.len();
                println!("Leaf {} at depth {}", &key, parents.len());
            }
        } else {
            break
        }

    }
    println!("Orbits: {}", orbitcount);
}

pub fn day6_2() {
//    let input_lines = "COM)B
//B)C
//C)D
//D)E
//E)F
//B)G
//G)H
//D)I
//E)J
//J)K
//K)L".split_ascii_whitespace();
    let input_lines = load_input_lines_str("day6.txt");
    let mut direct_orbits: HashMap<String,BTreeSet<String>> = HashMap::new();
    for line in input_lines.iter() {
        let parts: Vec<String> = line.split(')').map(|s| s.to_owned()).collect();
        let key = parts[0].to_owned();
        let entry = direct_orbits.entry(key).or_insert(BTreeSet::new());
        entry.insert(parts[1].to_owned());
    }
//    for (k, v) in direct_orbits.iter() {
//        println!("{})){:?}", k, v);
//    }

    let mut stack: Vec<(String,Vec<String>)> = vec![("COM".to_owned(), vec![])];
    let mut orbitcount = 0;
    let mut SAN_p = None;
    let mut YOU_p = None;
    loop {
        if let Some((key, parents)) = stack.pop() {
            if &key == &"SAN" {
                println!("SAN parents are {:?}", &parents);
                SAN_p = Some(parents.clone());
            } else if &key == &"YOU" {
                YOU_p = Some(parents.clone())
            }
            if let Some(children) = direct_orbits.get(&key) {
//                println!("Node {} with parents {:?}", &key, &parents);
                orbitcount += parents.len();
                let mut new_parents = parents.clone();
                new_parents.append(&mut vec![key]);
                for child in children.iter() {
                    stack.push((child.clone(), new_parents.clone()));
                }
            } else {
                orbitcount += parents.len();
//                println!("Leaf {} at depth {}", &key, parents.len());
            }
        } else {
            break
        }
    }
    if YOU_p.is_some() && SAN_p.is_some() {
        let my_parents = YOU_p.unwrap();
        let san_parents = SAN_p.unwrap();
        'outer: for (i, parent) in my_parents.iter().rev().enumerate() {
            for (j, shared) in san_parents.iter().rev().enumerate() {
                if &parent == &shared {
                    println!("{} steps from YOU to shared, {} from shared to SAN = {}", i, j, i+j);
                    break 'outer
                }
            }
        }
    }
    println!("Orbits: {}", orbitcount);
}