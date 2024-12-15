fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    // Safe way 1: cloning the vector
    let cloned_vec = vec.clone();
    let ptr = cloned_vec.as_ptr();
    println!("Pointer value: {:p}", ptr);
    unsafe {
        println!("Value at pointer: {}", *ptr);
    }
    drop(vec);
    // Safe way 2: using a reference
    let ptr2 = vec.as_ptr();
    println!("Pointer value: {:p}", ptr2);
    let ref_vec = &vec; 
    // you can use &vec[0] to access element of the vector, while the vector exists in memory
    println!("Value at pointer 2: {}", ref_vec[0]);
    
}