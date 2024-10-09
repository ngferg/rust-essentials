// signed ints   unsigned ints
// i8            u8
// i16           u16
// i32           i32
// i64           u64
// i128          u128

// use computer word size (hi perf)
// isize         usize

const SECONDS_IN_HOUR: i32 = 3_600; // underscore is ignored, can use as seperateor like commas for big nums

fn main() {
    println!("Hello, world!");
    println!("Invoking demo_ints! {}", demo_ints());
    println!("electron voltage: {:e}", demo_floats());
    println!("invoking simple types {}", simple_types());
    extras();
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

fn simple_types() -> bool {
    let is_murican = true;
    let can_sing = false;

    let is_murican_numeric = is_murican as i32;
    let can_sing_numeric = can_sing as i32;

    println!("bools! {} {} as int {} {}", is_murican, can_sing, is_murican_numeric, can_sing_numeric);
    println!("and {} or {} not {}", is_murican && can_sing, is_murican || can_sing, !is_murican);

    let middle_initial: char = 'G';
    let cool_guy: char = '😎';

    println!("chars! {} {}", middle_initial, cool_guy);

    return is_murican;
}

fn extras() {
    //infered types
    let a = 1;
    let b = 3.14;
    let c = true;

    //imutable by default, gotta declare mut to make it changable
    let mut d;
    d = 2;
    d = 4; // no ++ operator, gotta +=
    d += 1;

    // prefix with underscre for compiler to not warn you about unused vars
    let _f = 0;

    let g = 3.99;
    let h = g as i32; //conversions with as

    let num = "123";
    let num = num.parse::<i32>().unwrap();

    println!("{} {} {} {} {} {} {} {}", a, b, c, d, g, h, num, SECONDS_IN_HOUR);
}