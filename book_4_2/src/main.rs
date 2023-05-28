fn main() {
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;
    *x += 1;

    let r1: &Box<i32> = &x;
    let b: i32 = **r1;

    let r2: &i32 = &*x;
    let c: i32 = *r2;

    println!(
        "x: {} - a: {} - r1: {} - b: {} - r2: {} - c: {}",
        x, a, r1, b, r2, c
    );

    let mut vec: Vec<i32> = vec![0, 1, 2];
    vec.push(3);
    println!("-- vec {:?}", vec);

    let mut greet: Vec<char> = vec!['h', 'e', 'y'];
    let mut greet_short: Vec<char> = vec!['H', 'i'];

    fn ascii_capitalize(v: &mut Vec<char>) {
        let c = &v[0];
        println!();

        match c.is_ascii_lowercase() {
            true => {
                let up = c.to_ascii_uppercase();
                println!("{:?}", v);
                v[0] = up;
                println!("=> {:?} ✅", v);
            }
            false => {
                println!("{:?} ⭐", v);
            }
        }
    }

    ascii_capitalize(&mut greet);
    ascii_capitalize(&mut greet_short);
}
