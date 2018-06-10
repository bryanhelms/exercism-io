pub fn twofer(name: &str)-> String {
    // Using a mutable variable.
    let mut name_to_print = name;
    if name_to_print.is_empty() {
        name_to_print = "you";
    }

    return format!("One for {}, one for me.", name_to_print);
}
