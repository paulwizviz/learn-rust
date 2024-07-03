/*

Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. 
Floating-point numbers are represented according to the [IEEE-754 standard](https://en.wikipedia.org/wiki/IEEE_754).

*/

fn float32(){
    println!("-- Float 32 --");
    let size_of = std::mem::size_of::<f32>;
    println!("Size of float32 {}", size_of());

    let float32: f32 = -32.2;
    println!("Float32 {float32}");
    let float32_max: f32 = std::f32::MAX;
    println!("Float32 max is {float32_max}");
    let float32_min: f32 = std::f32::MIN;
    println!("Float32 min is {float32_min}");
    println!("Float32 infinity is {}", std::f32::INFINITY);
    println!("Float32 MAX_EXP is {}", std::f32::MAX_EXP);
    println!("Float32 NAN is {}", std::f32::NAN);
}

fn float64(){
    println!("-- Float 64 --");
    let size_of = std::mem::size_of::<f64>;
    println!("Size of float64 {}", size_of());

    let float64: f64 = -64.2;
    println!("Float64 {float64}");
    let float64_max: f64 = std::f64::MAX;
    println!("Float64 max is {float64_max}");
    let float64_min: f64 = std::f64::MIN;
    println!("Float64 min is {float64_min}");
    println!("Float64 infinity is {}", std::f64::INFINITY);
    println!("Float64 MAX_EXP is {}", std::f64::MAX_EXP);
    println!("Float64 NAN is {}", std::f64::NAN);
}


fn main() {
    float32();
    float64();
}
