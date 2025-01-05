// Primitive types

// fn main() {
//     // let x = -1; // This is i32, meaning signed int
//     let x: u8 = randint().wrapping_add(100);
//     println!("{x}");
//     let x: u8 = match randint().checked_add(100){
//         Some(num)=>num,
//         None => {
//             println!("Cannot add");
//             return;
//         }
//     };
//     println!("{x}");

// }

// fn randint() -> u8 {
//     200
// }


fn main(){
    let tup: (i32, f64, u8) = (500, 6.2, 3);
    let (x,y,z) = tup;
    println!("{}", tup.1);

    // Array
    let a:[i32; 5] = [1,2,3,4,5];
    let a= [3;3]; // [3,3,3]
    
}