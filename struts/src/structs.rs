#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn create_person(name: String, age: u8) -> Person{
    Person{
        name: name,
        age: age,
    }
}

pub fn instantiate(){

    let p = Person{
        name: String::from("John"),
        age: 21,
    };
    println!("Direct declarations {0:?}", p);

    let p_ptr = &Person{
        name: String::from("Jane"),
        age: 21,        
    };
    println!("Pointer {0:p} {0:?}", p_ptr);

    let p_created = create_person(String::from("Paul"), 57);
    println!("From creator {:?}", p_created);
}

