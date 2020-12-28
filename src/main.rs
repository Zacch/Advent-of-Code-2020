use std::time::Instant;

mod day17;

fn main() {
   let now = Instant::now();

   day17::day17();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
