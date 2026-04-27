const MAX_POINTS: u32 = 100_000;

fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let mut x = "5";
    println!("The value of x is: {}", x);
    x = "6";
    println!("The value of x is: {}", x);

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    let y = 5;
    println!("The value of y is: {}", y);

    {
        let y = 10;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y in the outer scope is: {}", y);

    let y = "shadowed";
    println!("The value of y after shadowing is: {}", y);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);

    // Data Types
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    // Integer types
    println!("Min & Max for i8: {} to {}", std::i8::MIN, std::i8::MAX);
    println!("Min & Max for u8: {} to {}", std::u8::MIN, std::u8::MAX);
    println!("Min & Max for i16: {} to {}", std::i16::MIN, std::i16::MAX);
    println!("Min & Max for u16: {} to {}", std::u16::MIN, std::u16::MAX);
    println!("Min & Max for i32: {} to {}", std::i32::MIN, std::i32::MAX);
    println!("Min & Max for u32: {} to {}", std::u32::MIN, std::u32::MAX);
    println!("Min & Max for i64: {} to {}", std::i64::MIN, std::i64::MAX);
    println!("Min & Max for u64: {} to {}", std::u64::MIN, std::u64::MAX);
    println!("Min & Max for i128: {} to {}", std::i128::MIN, std::i128::MAX);
    println!("Min & Max for u128: {} to {}", std::u128::MIN, std::u128::MAX);
    println!("Min & Max for isize: {} to {}", std::isize::MIN, std::isize::MAX);
    println!("Min & Max for usize: {} to {}", std::usize::MIN, std::usize::MAX);

    // Floating-point types
    println!("Min & Max for f32: {} to {}", std::f32::MIN, std::f32::MAX);
    println!("Min & Max for f64: {} to {}", std::f64::MIN, std::f64::MAX);

    // Boolean type
    let t = true;
    let f: bool = false;
    println!("The value of t is: {}", t);
    println!("The value of f is: {}", f);

    // Character type
    let c = 'z';
    let z: char = 'Z';
    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);

    // Tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let tup_1 = (500, 6.4, 1);
    println!("The first value of tup_1 is: {}", tup_1.0);
    println!("The second value of tup_1 is: {}", tup_1.1);
    println!("The third value of tup_1 is: {}", tup_1.2);

    // Array type
    let a = [1, 2, 3, 4, 5];
    println!("The first element of the array is: {}", a[0]);
    println!("The second element of the array is: {}", a[1]);

    let months = [
        "January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("Months: {}", months[5..8].join(", ")); // prints "June, July, August"

    let a:[i32;5] = [1,2,3,4,5];
    let b = [0;5];

    println!("The elements of array a are: {:?}", a);
    println!("The elements of array b are: {:?}", b);

    // Accessing array elements
    let first = a[0];
    let second = a[1];
    println!("The first element of array a is: {}", first);
    println!("The second element of array a is: {}", second);

    // Panics
    let index = 11;
    println!("The element at index {} is: {}", index, a[index]);

}
