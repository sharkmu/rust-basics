// 1. Hello World

// define a function called "hello_world with no parameters"
fn hello_world() { 
    // print out "Hello World!" in a new line
    println!("Hello World!");
}

// 2. Input

// import module
use std::io; 
fn input() 
{
    // mut stand for mutable, which means that we'll be able to change the variable's value later
    let mut input = String::new(); 

    println!("Please enter some text:");

    // this is an input. We will store the input in the "input" string. ".expect()" handles any upcoming errors
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // print out the "input" variable. We need the "{}", because we have an argument in it.
    println!("You entered: {}", input);
}

// 3. math operations 

fn math()
{
    // addition and subtraction

    // define "a", "b" variables and give them integer as value
    let a = 1;
    let b = 2;

    // give "c" variable the sum of "a" and "b" as value
    let c = a + b;

    // give "d" variable the subtract of "b" and "a" as value
    let d = b - a;

    // print out "c" and "d"
    println!("Sum: {}", c);
    println!("Subtraction: {}", d);

    // another way of getting the sum of "a" and "b" 
    println!("{}", a + b);

    // another way of getting the subtract of "b" and "a"
    println!("{}", b - a);

    // devision and multiplication

    // f64 means that it is a 64-bit floating-point number. If we want to make it a 32-bit, then just replace "64" with "32"
    let x = 2f64;
    let y = 3f64;

    // give "z" variable the product of "x" and "y"
    let z = x * y;

    // give "w" variable the devision of "y" and "x"
    let w = y / x;

    // print out "z" and "w"
    println!("Product: {}", z);
    println!("Devision: {}", w);
}

// this is the main function. We need this to be able to run the code.
fn main()
{
    // call the functions
    hello_world();
    input();
    math();
}