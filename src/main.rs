use std::time::Instant;

mod day07;

fn main() {
   let now = Instant::now();

   day07::day07();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
