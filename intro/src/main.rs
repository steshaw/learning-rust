use async_std;
use num_format::{Locale, ToFormattedString};

async fn hello(even_odd : &str, i : i32) {
    let answer = 1_234_5;
    println!("{}: Hello, {} {}", i, even_odd, answer.to_formatted_string(&Locale::en));
    assert_eq!(answer, 12_345);
}

#[async_std::main]
async fn main() {
    for i in 1..=3 {
      if  i % 2 == 0 {
        hello("even", i).await;
      } else {
        hello("odd", i).await;
      }
    }
}
