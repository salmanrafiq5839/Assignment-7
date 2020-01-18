mod lib;
fn main() {
    println!("Simple calculator");
    let mut no1=20;
    let mut no2=30;
  lib::calculator::calculator1(no1,no2);
  lib::calculator::calculator2(no1,no2);
}
