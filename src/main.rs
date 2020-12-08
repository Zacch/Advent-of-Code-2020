use std::time::Instant;

mod day08;

fn main() {
   let now = Instant::now();

   day08::day08();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
