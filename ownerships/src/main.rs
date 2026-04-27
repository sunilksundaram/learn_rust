

// Ownership is how rust manages memory & how it avoids a GC, yet able to make memory 
// safety guarantees. 
// related features: borrowing, slices, and how Rust lays data out in memory
// Ownership rules:
//  1. Each value in Rust has a variable that’s called its owner.
//  2. There can only be one owner at a time.
//  3. When the owner goes out of scope, the value will be dropped.

fn main() {
    show_scope();
    ownership_and_functions();
    ownership_returned_values();
    find_length();
    find_length_ref();
    change_ref_panic();
}

fn using_string_slice() {
    let my_str = String::from("hellow world"); // 
    // all these work
    let _s1 = first_word_string_slice(&my_str[0..8]);
    let _s2 = first_word_string_slice(&my_str[..]);
    let _s3 = first_word_string_slice(&my_str);

    let my_str_1 = "hello world"; // literals
    let _s4 = first_word_string_slice(&my_str_1[0..8]);
    let _s5 = first_word_string_slice(&my_str_1[..]);
    let _s6 = first_word_string_slice(&my_str_1);

    let _s7 = first_word_string_slice("hello world");
}

fn first_word_string_slice(s: &str) -> &str {
    // this will work on all kinds of strings
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }

    &s[..]
}

fn get_first_word_slice() {
    let s = String::from("hello world");

    let word = first_word_slice(&s);

    // s.clear(); // panics - mutable borrow occurs here, but it is already having another immutable borrow

    print!("First word: {word}")
}

fn first_word_slice(s: &String) -> &str { // String Slice - use &str
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        } 
    }

    &s[..]
}

fn string_slices() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello} :: {world}")
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn _cannot_mix_mutable_refs_fixed() {
    let mut s1 = String::from("hello"); // warning - variable need not be mutable

    let s2 = &s1;
    let s3 = &s1; // no problem
    println!("{s2}, {s3}"); // last usage of s1 & s2

    let s4 = &mut s1; // works! - the prev println was the last usage of immutable refs
    println!("{s4}"); 
}

fn _dangle() -> &String { // dangle returns a reference to a String

    let s: String = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope and is dropped, so its memory goes away. Danger!

fn _no_dangle() -> String {
    let s: String = String::from("hello"); // s is a new String
    s // instead send the String itself, not a reference
}

fn _cannot_mix_mutable_refs() {
    let mut s1 = String::from("hello"); // warning - variable need not be mutable

    let s2 = &s1;
    let s3 = &s1; // no problem
    //let s4 = &mut s1; // panics - cannot borrow `s1` as mutable because it is also borrowed as immutable
    
    println!("{s2}, {s3}");
    //println!("{s4}"); 
}

fn _cannot_have_multi_ref_solved() {
    let mut s1 = String::from("fellow");
    {
        let s2 = &mut s1;
        println!("{s2}");
    } // here s2 has gone out of scope, so s3 will work

    let s3 = &mut s1;
    println!("{s3}");
}

fn _cannot_have_multi_ref() {
    let mut s1 = String::from("fellow");
    let s2 = &mut s1;
    // let s3 = &mut s1; //panics

    //println!("{} {}", s2, s3);
    println!("{}", s2);
}

fn change_ref_panic() {
    let s1 = String::from("Hello");
    get_longer(&s1);
    println!("New String: {s1}")
}

fn get_longer(_s: &String) {
    // s.push_str("string"); // panics - `s` is a `&` reference, so it cannot be borrowed as mutable
    println!("We cannot borrow and make changes to it as mutable")
}

fn find_length_ref() {
    let s1 = String::from("hello");
    let len = calculate_len_ref(&s1); // pass a reference to s1
    println!("The length of '{}' is {}.", s1, len); // s1 is still valid here, because we passed a reference to s1, not ownership
}

fn calculate_len_ref(s: &String) -> usize {
    s.len() // we can call len() on a reference to a String, because it’s still a String
}

fn find_length() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_lenth(s1); // s1 is moved into calculate_lenth, and s2 takes ownership of the string returned by calculate_lenth
    println!("The length of '{}' is {}.", s2, len); // s2 is valid here, because it takes ownership of the string returned by calculate_lenth
}

fn calculate_lenth(s1: String) -> (String, usize) { 
    let len = s1.len(); // len() returns the length of a String
    (s1, len) // return the String and its length as a tuple
}

fn ownership_returned_values() {
    let s1 = gives_ownership();
    println!("s1: {}", s1); // s1 is valid here, because it takes ownership of the string returned by gives_ownership

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // println!("s2: {}", s2); // this will cause a compile error, because s2 is no longer valid after being moved into takes_and_gives_back
    println!("s3: {}", s3); // s3 is valid here, because it takes ownership of the string returned by takes_and_gives_back
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moves out to the calling function
}

fn ownership_and_functions() {
    let s = String::from("hello"); // s is valid from this point forward
    take_ownership(s); // s's value moves into the function...
    // ...and so is no longer valid here onwards
    // println!("s: {}", s); // this will cause a compile error, because s is no longer valid after the function call

    let x = 5; // x is valid from this point forward
    makes_copy(x); // x would move into the function, but i32 is Copy, so it’s okay to still use x afterward
    println!("x: {}", x); // this will work, because x is still valid after the function call
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
} // some_int goes out of scope here, but nothing special happens because it’s a Copy type

fn take_ownership(some_str: String) {
    println!("{}", some_str);
} // some_str goes out of scope and is dropped here

fn show_scope() {
    let s1 = String::from("hello");
    let s2 = s1; // now s1 is invalidated, and s2 is the owner of the string "hello"

    // println!("{}, world!", s1); // this will cause a compile error, because s1 is no longer valid
    println!("{}, world!", s2); // this will work, because s2 is the owner of the string "hello"
}
