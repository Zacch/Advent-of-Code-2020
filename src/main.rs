use std::time::Instant;

mod day15;

fn main() {
   let now = Instant::now();

   day15::day15();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
