fn main() {
    no_declared_return();
}

fn no_declared_return() {
    println!("no declared return function");
    println!("{}", return_declared()); // can't just println!(return_declared())
    param_detailed(true, 2);
    println!("{}", five());

    {
        fn return_declared() {
            println!("printing instead");
        }

        // uses the function declared in this scope - which shadows the external one
        return_declared();
    }

    // uses the function defined outside this main function - only one in scope
    println!("{}", return_declared());

    // // having below will shadow function declared outside of main which would
    // // cause above println! to panic as the below does not return a value
    // fn return_declared() {
    //     println!("printing instead 2");
    // }
    // // also cannot declare same function name twice in same block scope, 
    // // even if parameters and return differ. Issue is having two declarations 
    // // inside of the main function. Having one outside of main does not cause
    // // this issue when the same function name is used in a declaration inside main
    // fn return_declared(s: &String) -> String {
    //     println!("printing instead 2");
    // }
}

fn return_declared() -> String {
   String::from("some string")
}

fn param_detailed(x: bool, y: u32) {
    println!("{} and {}", x, y);
}

fn five() -> i32 {
    5
}