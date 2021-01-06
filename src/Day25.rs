pub fn day25() {
    let door_public_key = 13233401_i64;
    let card_public_key = 6552760_i64;

    let mut card_loop_size = 0;
    let mut value = 1;
    while value != card_public_key {
        card_loop_size += 1;
        value = (value * 7) % 20201227_i64;
    }
    let mut part1 = 1;
    for _ in 0..card_loop_size {
        part1 = (part1 * door_public_key) % 20201227_i64;
    }
    println!("Part 1: {}", part1);
}
