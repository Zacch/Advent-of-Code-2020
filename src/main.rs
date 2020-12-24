use std::time::Instant;

mod day24;

fn main() {
   let now = Instant::now();

   day24::day24();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
