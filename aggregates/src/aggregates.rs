pub fn array(){
    let a = [1, 2, 3];
    println!("The elements of a are {:?}", a);
    println!("Length of array is {}", a.len());

    let b:[i32; 3] = [0,1,3]; // fix array of i32 type with a size of 3 elements
    println!("The elements of b are {:?}", b);

    let byte_array = [b'A',b'c'];
    println!("The elements of byte_array is {:?}", byte_array) ;

    let repeated_elements = [3; 2]; // duplicate 2 elements of 3
    println!("First element of repeated_elements is {:?}", repeated_elements);

    let mut mutate_array = [3; 3];
    println!("Before mutation: the elements of mutateArray are {:?}", mutate_array);  
    mutate_array[0] = 1;
    println!("After mutation: the elements of mutateArray are {:?}", mutate_array);

}

pub fn slice(){

    let array = [1,2,3,4];
    let slice = &array[1..2];

    println!("slice {}", slice.len());
    println!("slice elements {:?}", slice);

}