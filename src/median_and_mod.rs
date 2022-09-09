mod median;
mod mod_;

pub fn print_median_and_mod(sorted_vec: &Vec<i32>) {
    println!("Vector: {:?}", sorted_vec);
    if sorted_vec.len() == 0 {
        println!("Can't measure empty array");
        return;
    }
    median::print_median(sorted_vec);
    mod_::print_mod(sorted_vec);
}
