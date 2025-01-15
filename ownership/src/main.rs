// fn main() {
//     let s1 = String::from("Hello");
//     // s1 contains ptr to heap where string is stored, length and capacity

//     let s2 = s1; // Ownership has been moved // Now s2 is the owner
//     // println!("{}", s1);
//     // s1 has been invalidated to prevent double clearing of heap memory

//     // To make deep copies of a string
//     // This is an expensive operation
//     // as it involves allocation in heap memory
//     let s3 = s2.clone();

//     let s = String::from("X");
//     takes_ownership(s);
//     // s.push_str("HAHA");
//     // s cannot be mutated as the ownership has been passed on to the function // DAMNNNNNN
// }

// fn takes_ownership(str:String){
//     println!("{str}");
// }

fn main() {
    let s1 = gives_ownership();

    let mut s2 = String::with_capacity(1);
    println!("{:p} {} {}",s2.as_ptr(), s2.len(),s2.capacity());
    s2.push_str("frffrefref");
    println!("{:p} {} {}",s2.as_ptr(), s2.len(),s2.capacity());

    let s3 = takes_and_gives_back(s2);
}
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
