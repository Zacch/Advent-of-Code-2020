use std::time::Instant;

mod day16;

fn main() {
   let now = Instant::now();

   day16::day16();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
