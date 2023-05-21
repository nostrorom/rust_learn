fn main() {
    let x: bool = true;
    read(x);

    let s1 = String::from("hello");
    let len1 = calculate_length(&s1);

    let s2 = String::from("hello !!");
    let len2 = calculate_length(&s2);

    let mut s3 = String::from("hello");
    change(&mut s3);

    println!("{}    --- length {}", s1, len1);
    println!("{} --- length {}", s2, len2);
    println!("changed string is --- {}", s3);

    let s = String::from("ciaoooooo mondo");

    println!(
        "first word: {} - length {}",
        first_word(&s),
        first_word(&s).len()
    );
}

fn read(y: bool) {
    if y {
        println!("{} is true :)", y);
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) -> () {
    s.push_str(" world");
}

fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
