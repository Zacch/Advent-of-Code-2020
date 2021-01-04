use std::time::Instant;

mod day20;

fn main() {
   let now = Instant::now();

   day20::day20();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
