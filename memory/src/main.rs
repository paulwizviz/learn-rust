mod pointers;

fn borrowing(){
    println!("-- Borrowing --");
    let x = String::from("hello");
    let y = &x;
    assert_eq!(&x,y);
    println!("Address of x is {:p} and the value of y is {:p}", &x, y);
}

fn shadowing(){
    println!("-- Shadowing --");
    let y = 1;
    let y = y + 5;
    let y = y + 1;
    assert_eq!(7, y);
    println!("Value of y is {y}");
}

fn main() {
    borrowing();
    shadowing();
    pointers::integers_pointer();
    pointers::string_pointers();
    pointers::box_pointer();
}
