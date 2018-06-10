pub fn twofer(name: &str)-> String {
    // Feels naive, not sure if this results in few allocations either.
    // Trying to avoid a mutable variable.
    let name_to_print;
    if name.is_empty() {
        name_to_print = "you";
    } else {
        name_to_print = name;
    }

    return format!("One for {}, one for me.", name_to_print);
}
