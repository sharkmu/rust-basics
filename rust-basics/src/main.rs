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

    // print out the "input" variable
    println!("You entered: {}", input);
}

// this is the main function. We need this to be able to run the code.
fn main()
{
    // call the functions
    hello_world();
    input();
}