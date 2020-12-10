use std::time::Instant;

mod day10;

fn main() {
   let now = Instant::now();

   day10::day10();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
