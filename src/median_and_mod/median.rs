pub fn print_median(sorted_vec: &Vec<i32>) {
    let middle_index = (sorted_vec.len() - 1) / 2;
    // no need to use .get since we made sure we do not deal with empty arrays
    println!("Median is {}", sorted_vec[middle_index]);
}
