mod the_cpp_lib;
use the_cpp_lib::{ TheMemory, init };

fn main() {
    println!("Hello, world!");
    let mut some_memory = TheMemory::default();
    unsafe {
      init(&mut some_memory, 9, 11);
    }
}
