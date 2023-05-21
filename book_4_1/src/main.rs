fn main() {
    let x: bool = true;
    read(x);

    let s1 = String::from("hello");
    let len1 = calculate_length(&s1);

    let s2 = String::from("hello !!");
    let len2 = calculate_length(&s2);

    let mut s3 = String::from("hello");
    change(&mut s3);

    println!("{} : length is {}", s1, len1);
    println!("{} : length is {}", s2, len2);
    println!("changed string is {}", s3);
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
