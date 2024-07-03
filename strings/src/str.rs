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
    let s = "This is a literal string";
    // The following appears to violate single ownership rule but not in the case of string literals
    let s1 = s; // ownership is not moved but the owner references the same fixed length string
    let s2 = s; // ownership is not moved but the owner references the same fixed length string
    println!("Ownership of literal strings s: {:p} s1 {:p} s2 {:p}", &s, s1, s2);
}