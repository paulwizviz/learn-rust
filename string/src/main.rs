mod strings;

pub fn characters(){
    println!("-- characters --");
    let c = 'z';
    println!("Character {}", c);
    let smile = '\u{1f600}';
    println!("Character {}", smile);
    let grin = '😊';
    println!("Character {}", grin);
}

fn main() {
    characters();
    strings::raw();
    strings::string_obj();
    strings::ownership();
}
