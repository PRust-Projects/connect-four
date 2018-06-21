pub fn get_other_token(token: &'static str) -> &'static str {
    if token == "X" {
        return "O";
    } else {
        return "X";
    }
}
