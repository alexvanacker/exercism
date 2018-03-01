pub fn verse(n: i32) -> String {
    let next_number = n - 1;
    let end_text = match n {
        0 => String::from("Go to the store and buy some more, 99 bottles of beer on the wall."),
        1 => String::from("Take it down and pass it around, no more bottles of beer on the wall."),
        2 => String::from("Take one down and pass it around, 1 bottle of beer on the wall."),
        _ => format!(
            "Take one down and pass it around, {} bottles of beer on the wall.",
            next_number
        ),
    };

    let start_text = match n {
        0 => String::from("No more bottles"),
        1 => String::from("1 bottle"),
        _ => format!("{} bottles", n),
    };

    format!(
        "{} of beer on the wall, {} of beer.\n{}\n",
        start_text,
        start_text.to_lowercase(),
        end_text
    )
}

pub fn sing(start: i32, end: i32) -> String {
    let mut result = String::new();
    for i in (end..(start + 1)).rev() {
        result.push_str(&verse(i));
        if i > end {
            result.push_str("\n");
        }
    }
    result
}
