pub fn reply(message: &str) -> &str {
    println!("Original message '{}'", message);
    let trimmed = message.trim();

    if trimmed.len() == 0 {
        return "Fine. Be that way!";
    }

    match( is_yelling(trimmed), is_question(trimmed)) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        (false, false) => "Whatever."
    }
}

pub fn is_yelling(message: &str) -> bool {
    let mut letters = message.chars().filter(|c| c.is_alphabetic());
    message.chars().any(|c| c.is_alphabetic()) && letters.all(|c| c.is_uppercase())
}

pub fn is_question(message: &str) -> bool {
    return message.ends_with('?');
}

pub fn contains_letters(message: &str) -> bool {

    for character in message.chars() {
        if character.is_alphabetic() {
            return true;
        }
    }
    false
}
