// Variables and constants

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");

//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     //Naming convention for rust is to use Upper case separated by underscores

// }

// Shadowing
fn main() {
    let x = 5;
    let x = x + 1; // 5 + 1

    {
        let x = x * 2; // 12
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}"); // 6
}
