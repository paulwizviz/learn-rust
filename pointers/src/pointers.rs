pub fn primitive(){
    let i = 8;
    let iptr = &i;
    let iptr1: *const i8 = &i;
    let iptr2: &i8 = &i;
    let value = *iptr;
    println!("Value of ipter by derefernce {}", value);
    println!("address of i {:p}", &i);
    println!("itpr {:p}", iptr);
    println!("itpr1 {:p}", iptr1);
    println!("itpr2 {:p}", iptr2);

    let mut v: i8 = 4;
    let iptr3: *mut i8 = &mut v;
    unsafe{
        println!("iptr3 addr {:p} value {}",iptr3, *iptr3);
        *iptr3 = 5;
        println!("iptr3 addr {:p} value {}", iptr3, *iptr3);
    }   
}

pub fn box_pointer(){
    let b1:Box<i8> = Box::new(8);
    let i :i8 = *b1;
    println!("i value {}", i);
}