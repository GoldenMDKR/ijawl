pub(crate) fn extract_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let end_extact = s
        .char_indices()
        .find_map(|(i, c)| if accept(c) { None } else { Some(i) })
        .unwrap_or_else(|| s.len());
    (&s[..end_extact], &s[end_extact..])
}

pub(crate) fn extract_digit(s: &str) -> (&str, &str) {
    extract_while(|c| c.is_ascii_digit(), s)
}

pub(crate) fn extract_operator(s: &str) -> (&str, &str) {
    match s.chars().next().unwrap() {
        '+' | '-' | '*' | '/' => (&s[..1], &s[1..]),
        _ => ("", s)
    }
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    extract_while(|c| c == ' ', s)
}

pub(crate) fn extract_next_token(s : &str) -> (&str, &str){
    let (_,s) = extract_whitespace(s);
    if s.len() == 0{
        return ("","");
    }
    let first = s.chars().next().unwrap();
    if first.is_ascii_digit() {
        extract_digit(&s)
    }
    else if first == '+' || first == '-' || first == '*' || first == '/' {
        extract_operator(&s)
    }
    else {
        ("","")
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_digit_none() {
        assert_eq!(extract_digit(""), ("", ""));
    }

    #[test]
    fn extract_digit_one() {
        assert_eq!(extract_digit("5+7"), ("5", "+7"));
    }

    #[test]
    fn extract_digit_several() {
        assert_eq!(extract_digit("53+7"), ("53", "+7"));
    }

    #[test]
    fn extract_digit_only() {
        assert_eq!(extract_digit("557"), ("557", ""));
    }

    #[test]
    fn extract_digit_not_digit() {
        assert_eq!(extract_digit("+7"), ("", "+7"));
    }

    #[test]
    fn extract_operator_none() {
        assert_eq!(extract_operator(""), ("", ""));
    }

    #[test]
    fn extract_operator_one() {
        assert_eq!(extract_operator("+7"), ("+", "7"));
    }

    #[test]
    fn extract_operator_several() {
        assert_eq!(extract_operator("//7"), ("//", "7"));
    }

    #[test]
    fn extract_operator_only() {
        assert_eq!(extract_operator("*"), ("*", ""));
    }

    #[test]
    fn extract_operator_not_digit() {
        assert_eq!(extract_operator("7"), ("", "7"));
    }

    #[test]
    fn extract_whitespace_none() {
        assert_eq!(extract_whitespace(""), ("", ""));
    }

    #[test]
    fn extract_whitespace_some() {
        assert_eq!(extract_whitespace("  7"), ("  ", "7"));
    }

    #[test]
    fn extract_all(){
        let s = "3+22 *-5";
        let (test,s) = extract_next_token(s);
        assert_eq!( test, "3");
        let (test,s) = extract_next_token(s);
        assert_eq!( test, "+");
        let (test,s) = extract_next_token(s);
        assert_eq!( test, "22");
        let (test,s) = extract_next_token(s);
        assert_eq!( test, "*");
        let (test,s) = extract_next_token(s);
        assert_eq!( test, "-");
        let (test,s) = extract_next_token(s);
        assert_eq!( test, "5");
        let (test,_) = extract_next_token(s);
        assert_eq!( test, "");
    }
}
