mod data_types;
mod aggregates;
mod primitives;
mod formatted_print;
mod strings;
mod pointers;

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
    println!("======");
    data_types::ownership();
    println!("=====");
    strings::raw();
    println!("=====");
    strings::string_obj();
    println!("=====");
    strings::ownership();
    println!("=====");
    aggregates::array();
    aggregates::slice();
    println!("======");
    formatted_print::string_fmt();
    println!("======");
    formatted_print::print_traits();
    println!("======");
    pointers::primitive();
    println!("======");
    pointers::box_pointer();
}
