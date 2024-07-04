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