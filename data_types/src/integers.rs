pub fn run(){

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
