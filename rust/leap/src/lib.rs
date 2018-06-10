pub fn is_leap_year(year: i32) -> bool {
    // h/t to Andrew for kicking off this idea
    let is_divisible_by = |num: i32| -> bool { year % num == 0 };
    return if is_divisible_by(400) { true }
        else if is_divisible_by(100) { false }
        else { is_divisible_by(4) };
}
