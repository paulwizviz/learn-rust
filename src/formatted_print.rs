pub fn string_fmt(){
    let world = "world";
    let mut str = format!("Hello {}", world);
    println!("{}",str);

    str = format!("Hello {}", 1);
    println!("{}",str);
}

pub fn print_traits(){
    let i = 123;
    println!("Integers literals {}", i);

    let f = 1.2;
    println!("Float literals {}", f);

    let c = 'a';
    println!("Float literals {}", c);

    let s = "Hello world";
    println!("Float literals {}", s);

    let b = true;
    println!("Float literals {}", b);

    let h:i64 = 0x0123456789ABCDEF;
    println!("Hex value {}", h);
    println!("Hex literals lower case {:x}", h);
    println!("Hex literals upper case {:X}", h);

    let o = 0o01234567;
    println!("Octal value {}", o);
    println!("Octal literals {:o}", o);

    let bin = 0b0101;
    println!("Binary value {}", bin);
    println!("Binary literals {:b}", bin);

    let exp = 1234.12;
    println!("Float exp value {}", exp);
    println!("Float exp literals lowercase {:e}", exp);
    println!("Float exp literals uppercase {:E}", exp);

    let value = 123;
    let p = &value;
    println!("Pointer value {}", p);
    println!("Pointer address {:p}", p);

    let array = ['a','b','c'];
    println!("Slice {:?}", array); // Print array value in debug form 
}