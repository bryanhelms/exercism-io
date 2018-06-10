pub fn is_leap_year(year: i32) -> bool {
    // Wanted to check out closures
    let is_divisible_by = |num: i32| -> bool { year % num == 0 };
    return is_divisible_by(4) && (!is_divisible_by(100) || is_divisible_by(400));
}
