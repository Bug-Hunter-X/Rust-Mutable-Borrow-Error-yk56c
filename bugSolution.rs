fn main() {
    let mut x = 5;
    { // This scope limits the lifetime of the mutable reference y
        let y = &mut x; // y is a mutable reference to x
        *y += 1;  // Modifying x through y
    }
    let z = &x; // z is an immutable reference to x
    println!("{}", *z); // Prints 6
    //Now the code will compile without any error
    let a = &mut x; // This will not cause any error now
    *a += 1;
    println!("{}", *a);
}