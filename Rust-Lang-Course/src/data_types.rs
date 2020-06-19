use std::mem; // namespace full of useful functions

fn data_types() {
    // println!("Hello, world!");
    let mut c = 123456789; // 32-bit signed i32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} after modification", c);
    // i8 u8 i16 u16 i32 u32 i64 u64
    let z: isize = 123; // isize/usize
                        //  These are integral data types which correspond to the size of the word so the size of the pointer
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {}, takes up {} bytes, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );

    let d: char = 'x'; // char is optional
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e: f32 = 2.5; // double precision, 8-bytes or 64-bits, f64
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    // true false
    let g = false;
    println!("g = {}, bytes = {}", g, mem::size_of_val(&g));
    let f = 4 > 0; // true
}
