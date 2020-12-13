use std::time::Instant;

mod day13;

fn main() {
   let now = Instant::now();

   day13::day13();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
