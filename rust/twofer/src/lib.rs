pub fn twofer(name: &str)-> String {
    // Saw this in someone else's solution. This is more like how I actually wanted to solve it.
    let name_to_print = if name.is_empty() {
        "you"
    } else {
        name
    };

    return format!("One for {}, one for me.", name_to_print);
}
