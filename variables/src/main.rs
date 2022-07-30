fn main() {
    let x = 5;
    let x = "now a string";

    let mut y = 2;
    y = 10_000; // equivalent to y = 10000;

    const SOME_CONSTANT: &str = "some value known at compile time";
    {
        const SOME_CONSTANT: &str = "will this work?";
        {
            const SOME_CONSTANT: u8 = 3;
        }
    }

    let a = [1, 2, 3];
    let [b, c, d] = a;
    println!("{}, {}, {}", b, c, d);

    let y = "string";
    let i = y;
    // string literals are copied not moved
    println!("{}, {}", y, i);
}
