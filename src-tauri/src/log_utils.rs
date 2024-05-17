pub fn print_pretty_log(message: &str, color: &str) {
    let reset = "\x1b[0m";
    let colored_message = match color {
        "red" => format!("\x1b[31m{}\x1b[0m", message),
        "green" => format!("\x1b[32m{}\x1b[0m", message),
        "blue" => format!("\x1b[34m{}\x1b[0m", message),
        _ => message.to_string(),
    };
    let log_line = format!("\n\t📢  {}\n", colored_message);
    println!("{}", log_line);
}