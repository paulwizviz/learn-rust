pub fn variables(){
    let int: isize = 32;
    // int = 50 -- not allowed
    println!("The value of non mutable int is {}", int);

    let mut mint: isize = 300;
    println!("The value of mutable int is {}", mint);
    mint = 100;
    println!("The value of mutable int after assign 100 is {}", mint);
}

pub fn pointers(){
    let int: isize = 32;

    let intptr: *const isize = &int;
    println!("Address of intptr {:p}", intptr);

}