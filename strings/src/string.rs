pub fn string_type(){
    println!("-- String Type --");
    let mut s = String::from("This is ");

    println!("New string: {}", s);
    s.push('a');
    s.push_str(" extra phrase.");
    println!("Add to string: {}", s);

    let rep = s.replace("a", "an");
    println!("Replaced `a` to `an`: {}", rep);

    let s1:&str = s.as_str();
    println!("Convert to `str` type: {}", s1);
}

pub fn length_and_capcity(){
    println!("-- String length and capacity --");
    let mut s = String::from("This is a beautiful world");
    println!("String: {} Capacity: {} Length: {}", s, s.capacity(), s.len());
    s.pop();
    s.remove(0);
    println!("String: {} Capacity: {} Length: {}", s, s.capacity(), s.len());
}

pub fn ownership(){
    println!("--- String ownership ---");
    let s2 = String::from("This is a string object");
    let s2_1 = s2;
    // s2 has moved so you can't reference it any more. If you reference it, it results in compiler error.
    // println!("Ownership of string object. S2: {} s2_1: {}", s2, s2_1)
    println!("Ownership of string object. S2: has moved to S2_1 from s2: {}", s2_1);
}

pub fn borrowing(){
    println!("--- String borrowing ---");
    let s = String::from("This is a string object");
    let str1 = &s; // str1 pointer value is the address of s
    let str2 = &s; // str2 pointer value is the address of s
    println!("Address of s {:p} value of str1 {:p} str2 {:p}", &s, str1, str2);
}