pub fn twofer(name: &str)-> String {
    if name.is_empty() {
        return format!("One for you, one for me.");
    }

    return format!("One for {}, one for me.", name);
}
