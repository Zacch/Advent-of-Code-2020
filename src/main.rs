use std::time::Instant;

mod day04;

fn main() {
   let now = Instant::now();

   day04::day04();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
