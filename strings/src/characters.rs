
pub fn literals(){
    println!("-- Character literals --");
    let size_of = std::mem::size_of::<char>;
    assert_eq!(4, size_of()); // four bytes

    let c = 'z';
    println!("Character {}", c);
    let smile = '\u{1f600}';
    println!("Character {}", smile);
    let grin = '😊';
    println!("Character {}", grin);
}

pub fn conversion(){
    println!("-- Character conversation --");
    let c = '\u{1f600}';
    println!("Assigned character: {} size: {}", c, std::mem::size_of_val(&c));
    let u_32 : u32 = c as u32;
    println!("Convert to u32 size: {} value: {} ", std::mem::size_of_val(&u_32), u_32);
    println!("Convert to char from u32 {}",std::char::from_u32(u_32).unwrap());
    let u_8 :u8 = c as u8;
    println!("Convert to u8 size: {} value: {}", std::mem::size_of_val(&u_8), u_8);
    let u_16 :u16 = c as u16;
    println!("Convert to u16 size: {} value: {}", std::mem::size_of_val(&u_16), u_16);
    let u_128 :u128 = c as u128;
    println!("Convert to u128 size: {} value: {}", std::mem::size_of_val(&u_128), u_128);
}