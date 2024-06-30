pub fn integers_pointer(){
    println!("--- Integers pointers ---");

    let x = 8;
    let y = &x;
    println!("x: {}, y: {}", x, y);

    let mut z = 10;
    let w = &mut z;    // Mutable reference to z
    *w += 5;
    assert_eq!(15, z);
}

pub fn string_pointers(){
    println!("--- String pointers ---");
    let st = String::from("hello");
    let st1 = &st;
    let st2 = &st;
    println!("Ownership of string object reference. st: {} st1: {} st2: {}", st, st1, st2);

    let mst = &mut String::from("world");
    println!("Mutable string pointer: {}",mst);
    let mst1 = mst;
    mst1.push('a');
    assert_eq!("worlda", mst1);
}

pub fn box_pointer(){
    println!("--- Box pointer ---");
    let b1:Box<i8> = Box::new(8);
    let i :i8 = *b1;
    println!("i value {}", i);

    let b2:Box<String> = Box::new(String::from("hello"));
    let s:String = *b2; 
    println!("s value {}", s);
}