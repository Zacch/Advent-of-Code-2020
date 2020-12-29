use std::time::Instant;

mod day18;

fn main() {
   let now = Instant::now();

   day18::day18();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
