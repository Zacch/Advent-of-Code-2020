use std::time::Instant;

mod day25;

fn main() {
   let now = Instant::now();

   day25::day25();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
