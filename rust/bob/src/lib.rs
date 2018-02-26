pub fn reply(message: &str) -> &str {
    println!("Original message '{}'", message);
    let trimmed = message.trim();

    if trimmed.len() == 0 {
        return "Fine. Be that way!";
    }

    // Yelling = All Caps 
    let uppercase = trimmed.to_uppercase();
    let is_yelling = uppercase == trimmed && contains_letters(trimmed);

    // Question = "?" at the end
    let is_question = trimmed.chars().last() == Some('?');

    if is_question && is_yelling {
        return "Calm down, I know what I'm doing!";
    }
    if is_yelling {
        return "Whoa, chill out!";
    }

    if is_question {
        return "Sure.";
    }

    return "Whatever.";
}

pub fn contains_letters(message: &str) -> bool {

    for character in message.chars() {
        if character.is_alphabetic() {
            return true;
        }
    }
    false
}
