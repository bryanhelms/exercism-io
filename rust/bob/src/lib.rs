pub fn reply(message: &str) -> &str {
    let (is_empty, is_yelled, is_question) = analyze_message(message);
    match (is_empty, is_yelled, is_question) {
        (true, _, _) => "Fine. Be that way!",
        (_, true, true) => "Calm down, I know what I'm doing!",
        (_, true, _) => "Whoa, chill out!",
        (_, _, true) => "Sure.",
        (_, _, _) => "Whatever."
    }
}

fn analyze_message(message: &str) -> (bool, bool, bool) {
    return if message.is_empty() { (true, false, false) }
        else {
        let mut alphabetics = message.chars().filter(|c| c.is_alphabetic());
        let is_yelled = alphabetics.any(|_| true) && alphabetics.all(|c| c.is_uppercase());
        let is_question = message.trim_right().chars().last() == Some('?');
        let is_empty = message.chars().all(|c| c.is_whitespace() || c.is_control());

        (is_empty, is_yelled, is_question)
    }
}
