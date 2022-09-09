mod employee_manager;
mod median_and_mod;
mod pig_latin;

fn main() {
    median_and_mod::print_median_and_mod(&vec![1, 1, 1, 2, 2, 3, 4]);
    median_and_mod::print_median_and_mod(&vec![1, 2, 3, 4, 5, 6]);
    median_and_mod::print_median_and_mod(&vec![9, 9, 10, 11, 12, 12]);
    median_and_mod::print_median_and_mod(&vec![0, 0]);
    median_and_mod::print_median_and_mod(&vec![1]);
    median_and_mod::print_median_and_mod(&vec![]);

    pig_latin::to_pig_latin("Hello, world!");
    pig_latin::to_pig_latin("Lorem ipsum dolor sit amet");
    pig_latin::to_pig_latin("Hell");
    pig_latin::to_pig_latin("The syntax for moving directories is the same as when moving files.
        In the following example, if the dir2 directory exists, the command will move dir1 inside dir2.
        If dir2 doesn’t exist, dir1 will be renamed to dir2");
    pig_latin::to_pig_latin("");
    pig_latin::to_pig_latin("    ");

    employee_manager::start();
}
