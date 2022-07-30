fn main() {
    let s = String::from("hello world");

    let first = &s[0..5];
    let second = &s[6..11];

    println!("{}, {}", first, second);

    let also_first = &s[..5];
    let also_second = &s[6..];

    println!("{}, {}", also_first, also_second);

    let all_of_s = &s[0..s.len()];
    let also_all_of_s = &s[..];

    println!("{}, {}", all_of_s, also_all_of_s);

    let a = [1, 2, 3, 4 , 5];
    let partial_ref = &a[1..3];

    println!("{:?} is equal to a reference to the [2, 3] part of {:?}", partial_ref, a);

    println!("{}", take_string_return_slice(&s));
    println!("{}", take_slice_return_slice(&s[..]));
    println!("{}", a_string_literal_is_a_string_slice("literal string"));

    let q = "some value".to_string();
    let w = &q[..4];
    println!("{}", w)
}

fn take_string_return_slice(s: &String) -> &str {
    &s[..5]
}

fn take_slice_return_slice(s: &str) -> &str {
    &s[..5]
}

// it is a slice of a specific point in the binary
fn a_string_literal_is_a_string_slice(s: &str) -> &str {
    s
}
