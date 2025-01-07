fn main() {
    let mut s1 = String::from("hello"); // s1 is the owner of the string in heap
    println!("{}", &s1); 
    let len = calculate_length(&s1); // A read-only reference is passed on to the function
    println!("The length of '{}' is {}.", s1, len);
    calculate_length1(&mut s1); // Passed a mutable reference to the function // Gave the permission to mutate
    println!("The new string is '{}'.", s1);
}   

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn calculate_length1(s: &mut String) {
    s.push_str("HHHA");
}

// ----------------- Rules of References -----------------

// A variable can have either one mutable reference or multiple immutable references but not both with the same scope
// Example 1:
// let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM

//     println!("{}, {}, and {}", r1, r2, r3);

// Example 2:
// let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{r1} and {r2}");
//     // variables r1 and r2 will not be used after this point

//     let r3 = &mut s; // no problem
//     println!("{r3}");


// ----------------- Dangling References -----------------

// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//    &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.