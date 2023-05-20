fn main() {
  let x = plus_one(five());

  println!("The value of x is {}", x);
  print_measurment(x, 'm');
}

fn print_measurment(value: i32, unit_label: char) {
  println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
  5
}

fn plus_one(x:i32) -> i32 {
  x + 1
}