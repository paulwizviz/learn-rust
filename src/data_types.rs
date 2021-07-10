pub fn integers(){
    let unsigned_int: u8 = 1;
    println!("unsigned 8-bit {}", unsigned_int);

    let signed_int: i8 = -1;
    println!("signed int 8-bit {}", signed_int);

    let visual_sep_int: i32 = 256_000;
    println!("visually separate int {}", visual_sep_int);

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

pub fn array(){
    let a = [1, 2, 3];
    println!("First element of a is {}", a[0]);
    println!("Length of array is {}", a.len());

    let b:[i32; 3] = [0,1,3]; // fix array of i32 type with a size of 3 elements
    println!("First element of b is {}", b[0]);

    let byte_array = [b'A',b'c'];
    println!("First element of byte_array is {}", byte_array[0]) ;

    let repeated_elements = [3; 2]; // duplicate 2 elements of 3
    println!("First element of repeated_elements is {}", repeated_elements[0]);
    println!("Second element of repeated_elements is {}", repeated_elements[1]);
}