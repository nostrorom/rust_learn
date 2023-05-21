fn main() {
    let condition: bool = true;

    let number: i32 = if condition { 6 } else { 5 };

    if condition {
        println!("condition : true");
    } else {
        println!("condition : false");
    }

    if number != 0 {
        println!("number is not zero");
    }
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let mut counter: i32 = 0;

    let result = loop {
        counter += 1;
        println!("counting {}...", counter);

        if counter == number {
            break counter * 100;
        }
    };

    println!("counter is : {}", counter);
    println!("result is : {}", result);
    println!();
    println!("--- loop ---");
    println!();

    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count {} :)", count);
        let mut remaining: i32 = 3;

        loop {
            if remaining == 0 {
                break;
            }
            if count == 5 {
                break 'counting_up;
            }
            println!("{}...", remaining);
            remaining -= 1;
        }

        count += 1;
    }
    println!("count is : {}", count);
    println!();
    println!("--- while ---");
    println!();

    let mut countdown: i32 = 5;

    while countdown != 0 {
        println!("🔥 {}", countdown);

        countdown -= 1;
    }
    println!("🔥🔥🔥🔥🔥 🚀");

    println!();
    println!("--- for ---");
    println!();

    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;

    while index < 5 {
        println!("while: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("for: {}", element);
    }

    for number in (1..=5).rev() {
        println!("for (range): {}", number);
    }
}
