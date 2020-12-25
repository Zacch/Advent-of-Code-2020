use std::time::Instant;

mod day22;

fn main() {
   let now = Instant::now();

   day22::day22();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
