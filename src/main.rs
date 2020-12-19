use std::time::Instant;

mod day14;

fn main() {
   let now = Instant::now();

   day14::day14();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
