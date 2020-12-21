use std::time::Instant;

mod day21;

fn main() {
   let now = Instant::now();

   day21::day21();

   println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
