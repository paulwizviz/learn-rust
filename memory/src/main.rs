mod pointers;

fn shadowing(){
    let y = 1;
    let y = y + 5;
    let y = y + 1;
    assert_eq!(7, y)
}

fn main() {
    shadowing();
    pointers::integers_pointer();
    pointers::string_pointers();
    pointers::box_pointer();
}
