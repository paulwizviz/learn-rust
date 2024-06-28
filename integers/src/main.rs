fn eight_bit(){
    println!("-- Eight bit --");
    let size_of: fn() -> usize = std::mem::size_of::<i8>;
    println!("Size of i8 {} in bytes",size_of());

    let eight: i8 = -8;
    println!("Eight bits integer: {eight}");
    let max_eight: i8 = std::i8::MAX; // 127
    println!("Max eight bits integer: {max_eight}");
    let min_eight: i8 = std::i8::MIN; // -128
    println!("Min eight bits integer: {min_eight}");

    let ueight: u8 = 8;
    println!("Unsign eight bits integer: {ueight}");
    let max_ueight: u8 = std::u8::MAX; // 255
    println!("Max unsigned eight bits integer: {max_ueight}");
    let min_ueight: u8 = std::u8::MIN; // 0
    println!("Min unsigned eight bits integer: {min_ueight}");
}

fn sixteen_bit(){
    println!("-- Sixteen bit --");
    let size_of: fn() -> usize = std::mem::size_of::<i16>;
    println!("Size of i16 {} in bytes",size_of());

    let sixteen: i16 = -16;
    println!("Sixteen bits integer: {sixteen}");
    let max_sixteen: i16 = std::i16::MAX; // 32767
    println!("Max sixteen bits integer: {max_sixteen}");
    let min_sixteen: i16 = std::i16::MIN; // -32768
    println!("Min sixteen bits integer: {min_sixteen}");

    let usixteen: u16 = 16;
    println!("Unsigned sixteen bits integer: {usixteen}");
    let max_usixteen: u16 = std::u16::MAX; // 65535
    println!("Max unsigned sixteen bits integer: {max_usixteen}");
    let min_usixteen: u16 = std::u16::MIN; // 0
    println!("Min unsigned sixteen bits integer: {min_usixteen}");
}

fn thirty_two_bit(){
    println!("-- Thirty-two bit --");
    let size_of: fn() -> usize = std::mem::size_of::<i32>;
    println!("Size of i32 {} in bytes",size_of());

    let thirty_two: i32 = -32;
    println!("Thirty-two bits integer: {thirty_two}");
    let max_thirty_two: i32 = std::i32::MAX; // 2147483647
    println!("Max thirty-two bits integer: {max_thirty_two}");
    let min_thirty_two: i32 = std::i32::MIN; // -2147483648
    println!("Min thirty-two bits integer: {min_thirty_two}");

    let uthirty_two: u32 = 32;
    println!("Unsigned thirty-two bits integer: {uthirty_two}");
    let max_uthirty_two: u32 = std::u32::MAX; // 429496729
    println!("Max unsigned thirty-two bits integer: {max_uthirty_two}");
    let min_uthirty_two: u32 = std::u32::MIN; // 0
    println!("Min unsigned thirty-two bits integer: {min_uthirty_two}");
}

fn sixty_four_bit(){
    println!("-- Sixty-four bit --");
    let size_of: fn() -> usize = std::mem::size_of::<i64>;
    println!("Size of i64 {} in bytes",size_of());

    let sixty_four: i64 = -64;
    println!("Sixty-four bits integer: {sixty_four}");
    let max_sixty_four: i64 = std::i64::MAX; // 9223372036854775807
    println!("Max sixty-four bits integer: {max_sixty_four}");
    let min_sixty_four: i64 = std::i64::MIN; // -9223372036854775808
    println!("Min sixty-four bits integer: {min_sixty_four}");

    let usixty_four: u64 = 64;
    println!("Unsigned sixty-four bits integer: {usixty_four}");
    let max_usixty_four: u64 = std::u64::MAX; // 18446744073709551615
    println!("Max unsigned sixty-four bits integer: {max_usixty_four}");
    let min_usixty_four: u64 = std::u64::MIN; // 0
    println!("Min unsigned sixty-four bits integer: {min_usixty_four}");
}

fn one_two_eight_bit(){
    println!("-- One two eight bit --");
    let size_of: fn() -> usize = std::mem::size_of::<i128>;
    println!("Size of i128 {} in bytes",size_of());

    let one_two_eight: i128 = -128;
    println!("One-two-eight bits integer: {one_two_eight}");
    let max_one_two_eight: i128 = std::i128::MAX; // 170141183460469231731687303715884105727
    println!("Max one-two-eight bits integer: {max_one_two_eight}");
    let min_one_two_eight: i128 = std::i128::MIN; // -170141183460469231731687303715884105728
    println!("Min one-two-eight bits integer: {min_one_two_eight}");

    let uone_two_eight: u128 = 128;
    println!("Unsigned one-two-eight bits integer: {uone_two_eight}");
    let max_one_two_eight: u128 = std::u128::MAX; // 340282366920938463463374607431768211455
    println!("Max unsigned one-two-eight bits integer: {max_one_two_eight}");
    let min_one_two_eight: u128 = std::u128::MIN; // 0
    println!("Min unsigned one-two-eight bits integer: {min_one_two_eight}");
}


fn main() {
   eight_bit();
   sixteen_bit();
   thirty_two_bit();
   sixty_four_bit();
   one_two_eight_bit();
}
