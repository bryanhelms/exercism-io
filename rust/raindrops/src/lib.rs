pub fn raindrops(n: usize) -> String {
    let sounds = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    let check_drops = | num_drops: usize | -> bool { n % num_drops == 0 };
    let mut measured_sound = String::new();

    for &(drops, sound) in sounds.iter() {
        if check_drops(drops) { measured_sound.push_str(sound); }
    }
    return if measured_sound.is_empty() { n.to_string() } else { measured_sound };
}
