fn main() {
    println!("Hello, world!");

    // this is immutable so the x will not change and give the error
    // let x = 1; // to mak the variable mutable us the keyword mut
    // println!("x is: {}", x);
    // x = 6;
    // println!("x is: {}", x);

    // in case you dont wants to make the varibale mutable, you can redeclare the varibale
    // let x = 1;
    // println!("x is: {}", x);
    // let x = 6;
    // println!("x is: {}", x);

    let mut x = 1;
    println!("x is: {}", x);
    x = 6;
    println!("x is: {}", x);

    {
        println!("x is: {}", x);
        x = x - 4;
    }

    x = x + 1;
    println!("x is: {}", x);
}
