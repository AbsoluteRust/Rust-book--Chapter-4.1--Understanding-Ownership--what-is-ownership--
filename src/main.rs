fn main() {
    //s is not valid here, it's not yet declared
    let s = "hello"; //s is valid from this point forward

    //do stuff with s
                    // this scope is now over, and s is no longer valid
    let s = String::from("hello"); //creating a `String` from a string literal using the `from` function

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); //this will print the mutable S string ("hello") appended with the pushed string ", world!"

    let x = 5;
    let y = x;

    // `String` version of the above integer example
    let s1 = String::from("hello");
    let s2 = s1; //points to the same space in the heap as s1 does, but also considers s1 as no longer valid to avoid "double free" errors, by not needing to push s1 out of scope later
                 // s1 is "moved" into s2

    //example of the `clone` method in action:
    let s1 = String::from("hello");
    let s2 = s1.clone();// a clone is a "deep copy" compared to the above which can be thought of as a "shallow copy", clones copy the data instead of the pointer to the data, basically

    println!("s1 = {}, s2 = {}", s1, s2);

    //stack only data: copy
    let x = 5;
    let y = x; // integers have known size and go on the stack instead of heap, so this does not make x invalid like the above s1/s2 example would. x was not moved to y.
               // clone wouldn't do anything different from the usual shallow copying and we can leave it out here.

    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

                                    
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn return_Values_And_Scope()
{

    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}



fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}