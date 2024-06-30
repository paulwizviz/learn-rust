fn ownership(){

    let s1 = "This is a raw string";
    let s1_1 = s1;
    println!("Ownership of raw strings S1: {} S2 {}", s1, s1_1);

    println!("--- change ownership of string value ---");
    let s2 = String::from("This is a string object");
    let s2_1 = s2;
    // s2 has moved so you can't reference it any more compiler error
    // println!("Ownership of string object. S2: {} s2_1: {}", s2, s2_1)
    println!("Ownership of string object. S2: has moved to S2_1, s2_1: {}", s2_1);
    
    println!("--- string references ---");

    let st = String::from("hello");
    let st1 = &st;
    let st2 = &st;
    println!("Ownership of string object reference. st: {} st1: {} st2: {}", st, st1, st2);

    println!("--- Mutating string references ---");

    let mut st = String::from("hello");    
    {
        let st0 = &mut st;
        st0.push('b');
        println!("St0 {}", st0);
    }
    let st1 = &mut st;
    st1.push('a');
    println!("Ownership of string object reference. st: moved to st1: {}", st1);

}


fn main() {
    ownership();
}
