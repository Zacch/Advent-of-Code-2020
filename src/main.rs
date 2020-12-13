use std::time::Instant;

mod day12;

fn main() {
   let now = Instant::now();

   day12::day12();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
