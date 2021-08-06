mod data_types;
mod aggregates;
mod primitives;
mod formatted_print;

fn main() {
    primitives::variables();
    println!("=====");
    primitives::pointers();
    println!("=====");
    data_types::integers();
    println!("=====");
    data_types::floats();
    println!("=====");
    data_types::characters();
    println!("=====");
    data_types::tuples();
    println!("=====");
    aggregates::array();
    aggregates::slice();
    println!("======");
    formatted_print::string_fmt();
    formatted_print::print_traits();
}
