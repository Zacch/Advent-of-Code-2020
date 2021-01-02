use std::time::Instant;

mod day23;

fn main() {
   let now = Instant::now();

   day23::day23();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
