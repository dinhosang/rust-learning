fn main() {
    let s = String::from("hello");
    let t = &s;

    // // t simply contains a pointer to s
    // // so value of s does not move to t, nor is it copied.
    println!("{}, {}", s, t);

    // // below will not work as regular references are not mutable
    // t.push_str(", world");

    let mut a = String::from("hello");
    let b = &mut a;

    b.push_str(", world");

    println!("{}", a);
    
    let c = String::from("hello");
    borrowing(&c);
    println!("the variable c still has ownership of {}", c);

    let mut d = String::from("hello");
    borrowing_and_modifying(&mut d);
    println!("variable d was modified to contain: {}", d);

    let mut v = String::from("hello mutable");

    let r1 = &mut v;
    let r2 = &mut v;

    // // below won't run as can only have a single active mutable reference in a scope
    // // assigned the mutable reference twice is fine, but trying to access the first
    // // will cause rust to panic.
    // println!("{}", r1);

    // below runs just fine though
    println!("{}", r2);

    let mut f = String::from("world");
    let p1 = &f;
    let p2 = &f;

    println!("{}, {}", p1, p2);
    let p3 = &mut f;

    // // below would cause above mutable reference to make Rust panic
    // // can't have a mutable reference assignment if an immutable reference variable
    // // is accessed in the same scope after the mutable reference assignment.
    // println!("{}, {}", p1, p2);
    
    p3.push_str(" say hello");
    println!("{}", p3);
}

fn borrowing(s: &String) {
    println!("printing a reference to a variable containing: {}", s);
}

fn borrowing_and_modifying(s: &mut String) {
    s.push_str(" world, again");
}

// // below will cause rust to panic as the reference returned will point to no value
// // the string s will be dropped after the function call completes leading to an
// // empty reference.
// fn no_dangling_references_allowed() -> &String {
//     let s = String::from("hello");
//     &s
// }
