fn main() {
    let x = [1, 2, 3];
    let y = x;

    println!("{:?}, {:?}", x, y);

    let a = [String::from("string"), String::from("string2")];
    let b = a;

    // below causes rust to panic as a contains values that exist on the heap
    // meaning value on a is Moved, not Copied.
    // println!("{:?}, {:?}", a, b);

    let c = 1;
    let d = c;

    println!("{:?}, {:?}", c, d);

    let e = ["literal string", "&str"];
    let f = e;

    println!("{:?}, {:?}", e, f);

    let g = "literal";
    let h = g;

    println!("{:?}, {:?}", g, h);

    let mut t = (1,2);
    let mut r = t;

    r.0 = 3;
    println!("{}, {}", t.0, r.0);

    let mut q = 4;
    example(&mut q);
    println!("{}", q);
}

fn example(x: &mut i32) {
    *x += 1;
}