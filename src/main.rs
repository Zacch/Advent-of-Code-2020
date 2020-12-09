use std::time::Instant;

mod day09;

fn main() {
   let now = Instant::now();

   day09::day09();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
