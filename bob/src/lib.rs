pub fn reply(message: &str) -> &str {
    let msg_no_spaces: String = message.trim()
        .split(|c: char| c.is_whitespace())
        .collect::<Vec<&str>>()
        .join("");

    // useful when debugging
    // println!("msg_no_spaces: {msg_no_spaces}");

    let all_caps = msg_no_spaces.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .all(|c| c.is_ascii_uppercase()) && 
        msg_no_spaces.chars()
            .any(|c| c.is_ascii_alphabetic());

    let q_mark = msg_no_spaces.ends_with("?");

    let silence = msg_no_spaces == "";

    if all_caps && q_mark {
        return "Calm down, I know what I'm doing!";
    }
    if q_mark {
        return "Sure.";
    }
    if all_caps {
        return "Whoa, chill out!";
    }
    if silence {
        return "Fine. Be that way!";
    }

    "Whatever."
}
