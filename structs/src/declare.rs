#[derive(Debug)]
struct Human {
    name: String,
    age: u8,
}

pub fn run_atomic(){

    println!("-- Atomic struct declaration --");

    let p = Human{
        name: String::from("John"),
        age: 21,
    };
    println!("Direct declarations: {:?} Name is: {} Age is: {}", p, p.name, p.age);

    let p_ptr = &Human{
        name: String::from("Jane"),
        age: 21,        
    };
    println!("Pointer: {0:p} Value: {0:?}", p_ptr);
    println!("Direct declarations: {:?} Name is: {} Age is: {}", p_ptr, p_ptr.name, p_ptr.age);

}

#[derive(Debug)]
struct Dog {
    name: String,
}

#[derive(Debug)]
struct LivingThing{
    human: Human,
    dog: Dog,
}

pub fn run_embedded(){
    println!("-- Embedded struct --");
    let d = Dog{
        name: String::from("woofie"),
    };
    let h = Human{
        name: String::from("alice"),
        age: 12,
    };
    let l = LivingThing{
        dog: d,
        human: h,
    };
    println!("Living thing: {:?} Dog: {:?} Human: {:?}", l, l.dog, l.human);
}