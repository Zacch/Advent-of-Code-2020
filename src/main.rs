use std::time::Instant;

mod day11;

fn main() {
   let now = Instant::now();

   day11::day11();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
