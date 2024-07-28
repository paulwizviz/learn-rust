
pub fn tuples(){
    let tup: (i32, f64, u8) = (100, 1.2, 1);
    let (x, y, z) = tup;
    println!("Tuple (x,y,z) = (100, 1.2, 1) value of x is {}", x);
    println!("Tuple (x,y,z) = (100, 1.2, 1) value of y is {}", y);
    println!("Tuple (x,y,z) = (100, 1.2, 1) value of y is {}", z);
    println!("-------");
    println!("let tup: (i32, f64, u8) = (100, 1.2, 1)  tup.0 {}", tup.0)
}

pub fn ownership(){
    println!("-- Tuple --");

    let c = (1, "hello", 2.5, true);
    let d = c; // move permitted
    println!("c: {:?} d: {:?}",c,d);

/*
    let x = (1, String::from("hello"), 2.5); // Tuple contains object type
    let y = x; // Move not permitted <- compiler error
    println!("x: {:?} y: {:?}",x,y);
*/

    let b = (String::from("hello"), String::from("world"));
    let c = &b; // borrowed
    println!("b: {:?} c: {:?}", b, c);
}

fn main(){
    tuples();
}