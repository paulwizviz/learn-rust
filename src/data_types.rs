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

    let mut mutate_array = [3; 3];
    println!("Before mutation: first element of mutateArray is {}", mutate_array[0] );
    println!("Before mutation: second element of mutateArray is {}", mutate_array[1] );   
    mutate_array[0] = 1;
    println!("After mutation: first element of mutateArray is {}", mutate_array[0] );
    println!("After mutation: second element of mutateArray is {}", mutate_array[1] );

}

pub fn slices(){

    let array = [1,2,3,4];
    let slice = &array[1..2];

    println!("slice {}", slice.len());
    println!("slice first element {}", slice[0]);

}