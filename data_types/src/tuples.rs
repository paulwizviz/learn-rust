pub fn run(){
    let tup: (i32, f64, u8) = (100, 1.2, 1);
    let (x, y, z) = tup;
    println!("Tuple (x,y,z) = (100, 1.2, 1) value of x is {}", x);
    println!("Tuple (x,y,z) = (100, 1.2, 1) value of y is {}", y);
    println!("Tuple (x,y,z) = (100, 1.2, 1) value of y is {}", z);
    println!("-------");
    println!("let tup: (i32, f64, u8) = (100, 1.2, 1)  tup.0 {}", tup.0)
}