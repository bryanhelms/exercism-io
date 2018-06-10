pub fn twofer(name: &str)-> String {
    return format!("One for {}, one for me.", if name.is_empty() { "you" } else { name });
}
