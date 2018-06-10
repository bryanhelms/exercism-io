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
    return {
        let is_empty = message.trim().is_empty();
        let is_yelled = message.chars().any(|c| c.is_alphabetic())
            && message.chars().all(|c| c.is_uppercase() || !c.is_alphabetic());
        let is_question = message.trim_right().chars().last() == Some('?');

        (is_empty, is_yelled, is_question)
    }
}
