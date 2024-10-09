// signed ints   unsigned ints
// i8            u8
// i16           u16
// i32           i32
// i64           u64
// i128          u128

// use computer word size (hi perf)
// isize         usize

fn main() {
    println!("Hello, world!");
    println!("Invoking demo_ints! {}", demo_ints());
    println!("electron voltage: {:e}", demo_floats());
}

fn demo_ints() -> i64 {
    let sixty_four_bit_int: i64 = 1234;
    let a1: i8 = 127;
    let a2: i32 = 0xFF;
    let b1: usize = 1234567;

    println!("some ints {} {} {} {}", sixty_four_bit_int, a1, a2, b1);

    return sixty_four_bit_int;
}

fn demo_floats() -> f64 {
    let pi: f32 = 3.14159;
    let pi2: f64 = 3.14159;
    let sci: f64 = -1.602176e-16;

    println!("pi's: {:.2} {}", pi, pi2);
    println!("left: {:<10.2} right: {:>10.2}", pi2, pi2);
    println!("padded left: {:0<10.2} right: {:0>10.2}", pi2, pi2);

    return sci;
}
