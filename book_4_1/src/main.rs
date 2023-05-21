fn main() {
    let x: bool = true;
    read(x);

    let s1 = String::from("hello");
    let s2 = String::from("hello !!");

    let len1 = calculate_length(&s1);
    let len2 = calculate_length(&s2);

    println!("{} : length is {}", s1, len1);
    println!("{} : length is {}", s2, len2);
}

fn read(y: bool) {
    if y {
        println!("{} is true :)", y);
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
