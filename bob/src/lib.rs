pub fn reply(message: &str) -> &str {
    let message: &str = message.trim();
    let traits: (bool, bool, bool) = (
        message.ends_with("?"),
        match message.find(char::is_alphabetic) {
            Some(..) => message == message.to_uppercase(),
            _ => false
        },
        message.is_empty()
    );

    match traits {
        (true, true, ..) => "Calm down, I know what I'm doing!",
        (true, ..) => "Sure.",
        (false, true, ..) => "Whoa, chill out!",
        (.., true) => "Fine. Be that way!",
        _ => "Whatever."
    }
}
