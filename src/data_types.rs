pub fn integers(){

    let int: usize = 32;
    println!("unsigned architecture dependent int {}", int);

    let int: isize = -32;
    println!("signed architecture dependent int {}", int);

    let unsigned_int8: u8 = 255;
    println!("unsigned int8 {}", unsigned_int8);

    let signed_int8: i8 = -128;
    println!("signed int8 {}", signed_int8);

    let signed_int32: i32 = 256_000;
    println!("Signed int32 {}", signed_int32);

    let unsigned_int32: u32 = 256_000;
    println!("Unsigned int32 {}", unsigned_int32);

    let signed_int64: i64 = -256_000;
    println!("Unsigned int64 {}", signed_int64);

    let unsigned_int64: u64 = 256_000;
    println!("Unsigned int64 {}", unsigned_int64);

    let signed_int128: i128 = -256_000;
    println!("Unsigned int128 {}", signed_int128);

    let unsigned_int128: u64 = 256_000;
    println!("Unsigned int128 {}", unsigned_int128);

    let binary_type: i8 = 0b0110;
    println!("binary type {}", binary_type);

    let byte_type: u8 = b'A';
    println!("byte type {}", byte_type);
}

pub fn floats(){
    let float_32: f32;
    float_32 = 1.0;
    println!("float32 {}", float_32)
}

pub fn characters(){
    let c = 'z';
    println!("Character {}", c);
    let smile = '\u{1f600}';
    println!("Character {}", smile);
    let grin = '😊';
    println!("Character {}", grin);
}

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
    let i = 1;
    let i1 = i;
    println!("i: {} i1: {}", i, i1);
}
