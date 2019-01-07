extern crate rand;
use rand::Rng;
use std::collections::HashMap;

static NAMES: &'static [&'static str] = &[
    "Sean",
    "Ryan",
    "Apeach",
    "Muzi",
    "Frodo",
    "Tube",
];

fn main() {
    let mut current = 0;
    let max = 10000;

    let mut container = Vec::new();

    while current < max {
        current = current + 1;
        container.push(Item::new(current, get_random_name()));
    }

    println!("Container length: {}", container.len());

    let mut name_counts: HashMap<String, i32> = HashMap::new();

    container
        .iter()
        .for_each(|x| {
            let count = name_counts.entry(x.name.to_string()).or_insert(0);
            *count += 1;
        });

    get_stats(&mut name_counts);
    println!("Result: {:?}", name_counts);
}

fn get_random_name() -> String {
    let rand_index = rand::thread_rng().gen_range(0, NAMES.len());
    NAMES[rand_index].to_string()
}

struct Item {
    name: String,
    id: i32,
}

impl Item {
    pub fn new(id: i32, name: String) -> Item {
        Item {
            id,
            name,
        }
    }
}

fn get_stats(name_map: &mut HashMap<String, i32>) {
    let mut total: i32 = 0;
    let mut max: i32 = 0;
    let mut max_name: String = "".to_string();

    let mut min: i32 = 0;
    let mut min_name: String = String::from("");

    for (key, value) in name_map {
        total += *value;
        if value > &mut max {
            max = *value;
            max_name = key.to_string();
        }

        if min == 0 {
            min = *value;
            min_name = key.to_string();
        } else {
            if *value < min {
                min = *value;
                min_name = key.to_string();
            }
        }
    }

    println!("Total counts = {}", total);
    println!("Min = {}", min);
    println!("Min name = {}", min_name);
    println!("Max = {}", max);
    println!("Max name = {}", max_name);
    println!("Range: {}", max - min);
}

