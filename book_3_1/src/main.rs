fn main() {
    println!("-----");
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);
    
    // other example 
    println!("-----");
    
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("inner - y = {}", y);
    }
    println!("outer - y = {}", y);
    println!("-----");
}