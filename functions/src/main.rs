fn main() {
    println!("Hello, world!");
    another_function();
    yet_another_function(42);
    _unused_function();
    print_labeled_measurement(10, 'm');
    mypanics();
    expressions();

    println!("The value returned by five() is: {}", five());
    println!("The value returned by plus_one(5) is: {}", plus_one(5));
}

fn another_function() {
    println!("This is another function.");
}

fn yet_another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn _unused_function() {
    println!("This function is not used.");
}

fn print_labeled_measurement(val:i32, unit:char) {
    println!("The measurement is: {}{}", val, unit);
}

// Statements are instructions that perform some action and do not return a value. 
// Expressions evaluate to a resulting value. 
// For ex., in the line `let x = 5 + 6;`, `5 + 6` is an expression that evaluates to `11`, 
// and the entire line is a statement that assigns the value `11` to the variable `x`.

// Cannot assign let statement to a variable because it is a statement, not an expression.
fn mypanics() {
    println!("These will panic!");
    // let y = (let x = 5); // This will cause a compile-time error
    // let y = let x = 5; // This will also cause a compile-time error
}

// Expressions evaluate to a value, so you can assign them to variables.
// let x = 5; // This is a statement that declares a variable `x` and assigns it the value `5`.
// let y = 5 + 6; // This is a statement that declares a variable `y` and assigns it the value of the expression `5 + 6`, which evaluates to
// Calling a function, macro, new scope block code within {} are also expressions.

fn expressions() {
    let x = 5; // Statement
    let y = 5 + 6; // Statement with an expression on the right-hand side
    let z = {
        let x = 3;
        x + 1 // This is the last expression in the block, so it will be returned and assigned to `z`
    };
}

// This is an expression that evaluates to the value `5`, which is returned by the function
// By default the last value evaluated in a fn is returned, so we can omit the `return` keyword and the semicolon.
fn five() -> i32 {
    5 
}

// This expression evaluates to `x + 1`, which is returned by the function. There is 
// no need for a semicolon at the end of the expression, because that would turn it 
// into a statement and prevent it from returning a value.
fn plus_one(x: i32) -> i32 {
    x + 1 
}