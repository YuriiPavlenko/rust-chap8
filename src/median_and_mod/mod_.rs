use std::collections::HashMap;

pub fn print_mod(sorted_vec: &Vec<i32>) {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for num in sorted_vec {
        let count = map.entry(*num).or_insert(0);
        *count += 1
    }
    let mut max: usize = 0;
    let mut mods: Vec<i32> = Vec::new();
    for (k, v) in map {
        if v > max {
            max = v;
            mods.clear();
            mods.push(k);
        } else if v == max {
            mods.push(k);
        }
    }
    mods.sort();
    println!("Mod is {:?}", mods);
}
