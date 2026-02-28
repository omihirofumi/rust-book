fn main() {
    let s = "Hello, world!";
    let ptr = s.as_ptr();
    let len = s.len();

    println!("s = {:?}", s);
    println!("ptr = {:p}", ptr);
    println!("len = {}", len);

    println!("ptr (usize) = 0x{:x}", ptr as usize);
}
