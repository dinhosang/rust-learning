fn main() {
    let condition = true;

    if condition {
        println!("this will print 1");
    } else {
        println!("this will not print");
    }

    let number = if condition { 5 } else { 6 };

    println!("number is equal to 5, see - {}", number);

    if number < 5 {
        println!("will not print");
    } else if number > 5 {
        println!("will not print");
    } else {
        println!("this will print 2");
    }

    let mut counter = 0;

    loop {
        counter += 1;

        if counter > 3 {
            break;
        }
    }

    println!("counter is at four: {}", counter);

    counter = 0;

    let result = loop {
        counter += 1;

        if counter > 3 {
            break counter * 2;
        }
    };

    println!("result holds the number eight: {}", result);

    counter = 0;

    while counter < 3 {
        println! {"counter is currently: {}", counter}
        counter += 1;
    }

    let a = [10, 20, 30];

    for element in a.iter() {
        println!("current element is: {}", element);
    }

    for element in (1..4).rev() {
        println!("current element is: {}", element);
    }
}
