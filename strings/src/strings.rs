pub fn raw(){
    println!("-- String raw literals --");
    let line_break: &str = r"This is a very long string,
that spans across multiple lines."; // with line break.
    println!("With line break: {}", line_break);

    let string_with_escape = "\"Hello\" \\world";
    println!("With quotes and escape: {}", string_with_escape);

    let string_with_backslash = r"C:\Users\name\Documents\file.txt";
    println!("String with backlash: {}", string_with_backslash);

    let rhash_string_with_quote = r#""Hello World", he said"#;
    println!("String with quote: {}", rhash_string_with_quote);

    let complex_string = r###"This is a "complex string" with # to form a sentence"###;
    println!("With quotes and hash: {}", complex_string);

    let json_string = r#"{"name": "John", "age": 30, "city": "New York"}"#;
    println!("JSON string: {}", json_string);
}

pub fn ownership_raw(){
    println!("-- Raw string ownership --");
    let s1 = "This is a raw string";
    let s1_1 = s1;
    println!("Ownership of raw strings S1: {} S2 {}", s1, s1_1);
}

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

pub fn substring(){
    println!("-- Sub string --");
    let long_string = r"This is a very long string,
that spans across multiple lines.";
    let sub_string = &long_string[2..4];
    println!(r#"Substring 2..4 is: "{}""#, sub_string);
}

pub fn string_ownership(){
    println!("--- String ownership ---");
    let s2 = String::from("This is a string object");
    let s2_1 = s2;
    // s2 has moved so you can't reference it any more compiler error
    // println!("Ownership of string object. S2: {} s2_1: {}", s2, s2_1)
    println!("Ownership of string object. S2: has moved to S2_1, s2_1: {}", s2_1);
}