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
}

fn demo_ints() -> i64 {
    let sixty_four_bit_int: i64 = 1234;
    let a1: i8 = 127;
    let a2: i32 = 0xFF;
    let b1: usize = 1234567;

    println!("some ints {} {} {} {}", sixty_four_bit_int, a1, a2, b1);

    return sixty_four_bit_int;
}
