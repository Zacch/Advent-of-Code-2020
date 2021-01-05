use std::time::Instant;

mod day19;

fn main() {
   let now = Instant::now();

   day19::day19();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
