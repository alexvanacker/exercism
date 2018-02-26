pub fn reply(message: &str) -> &str {
    println!("Original message '{}'", message);
    let trimmed = message.trim();

    if trimmed.len() == 0 {
        return "Fine. Be that way!";
    }

    // Yelling = All Caps 
    let is_yelling = is_yelling(trimmed);

    // Question = "?" at the end
    let is_question = is_question(trimmed);

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
