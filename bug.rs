fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let ptr = vec.as_ptr();
    drop(vec);
    println!("Pointer value: {:p}", ptr);
    unsafe {
        println!("Value at pointer: {}", *ptr);
    }
}