fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    *y += 1;  // Modifying x through y
    let z = &x; // z is an immutable reference to x
    println!("{}", *z); // Prints 6
    //The problem occurs when another mutable reference tries to access x while z is still in scope.
    //This leads to compile-time error.
    let a = &mut x; // This will cause a borrow checker error 
}