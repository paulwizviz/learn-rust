pub fn literals(){
    println!("-- String literals --");
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

pub fn substring(){
    println!("-- Sub string --");
    let long_string = r"This is a very long string,
that spans across multiple lines.";
    let sub_string = &long_string[2..4];
    println!(r#"Substring 2..4 is: "{}""#, sub_string);
}

pub fn ownership(){
    println!("-- Literal string ownership --");
    let s1 = "This is a raw string";
    let s1_1 = s1;
    println!("Ownership of raw strings S1: {} S2 {}", s1, s1_1);
}