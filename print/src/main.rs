fn string_fmt(){
    let world= "world";
    let mut str: String = format!("Hello {world}");
    println!("{}",str);

    str = format!("Hello {}", 1);
    println!("{}",str);
}

fn main() {
    string_fmt()
}
