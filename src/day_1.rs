use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn day_1() {
    println!("=== day 1 ===");

    let mut group1: Vec<u32> = vec![];
    let mut group2: Vec<u32> = vec![];

    // Open the file
    let file = File::open("input/day1.txt").unwrap();
    let reader = io::BufReader::new(file);

    // Read the file line by line
    for line in reader.lines() {
        let line = line.unwrap();

        let ids: Vec<&str> = line.split_whitespace().collect();
        let group_1_id = ids[0].parse::<u32>().expect("Invalid number");
        let group_2_id = ids[1].parse::<u32>().expect("Invalid number");

        group1.push(group_1_id);
        group2.push(group_2_id);
    }

    group1.sort();
    group2.sort();

    let mut total_diff: u32 = 0;

    for (index, value) in group1.iter().enumerate() {
        let d = value.abs_diff(group2[index]);

        total_diff += d;
    }

    println!("part 1: total dif: {total_diff}");

    part_2(group1, group2);
}

pub fn part_2(group1: Vec<u32>, group2: Vec<u32>) {
    // dbg!(group1);
    // dbg!(group2);

    let mut total_s_score: u64 = 0;

    // store a cache of similarity scores for each unique ID
    let mut s_cache: HashMap<&u32, u64> = HashMap::new();

    group1.iter().for_each(|id| {
        // first check if this ID is in the cache and if so use the cached s_score
        // println!("scache len: {}", s_cache.len());
        if s_cache.contains_key(id) {
            total_s_score += s_cache.get(id).unwrap();
        } else {
            let occurances = count_occurances(*id, &group2);
            let s_score = (*id * occurances as u32) as u64;
            s_cache.insert(id, s_score);

            total_s_score += s_score;
        }
    });

    println!("part 2: total s_score: {total_s_score}");
}

fn count_occurances(value: u32, group2: &Vec<u32>) -> usize {
    let occurances: Vec<u32> = group2
        .iter()
        .filter(|id| **id == value)
        .map(|_| 0)
        .collect();

    occurances.len()
}
