pub fn raindrops(n: usize) -> String {
    let check_drops = | num_drops: usize | -> bool { n % num_drops == 0 };
    let mut sound = String::new();
    if check_drops(3) { sound.push_str("Pling"); }
    if check_drops(5) { sound.push_str("Plang"); }
    if check_drops(7) { sound.push_str("Plong"); }
    if sound.is_empty() { sound = n.to_string(); }

    return sound;
}
