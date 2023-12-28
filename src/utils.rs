pub(crate) fn extract_while(accept: impl Fn(char) -> bool, s: &str) -> (&str,&str){
    let end_extact = s
        .char_indices()
        .find_map(|(i,c)| 
            if accept(c) {
                None
            } else {
                Some(i)
            })
        .unwrap_or_else(|| s.len());
    (&s[..end_extact], &s[end_extact..])
}

pub(crate) fn extract_digit(s: &str) -> (&str,&str){
    extract_while(|c| c.is_ascii_digit(), s)
}

pub(crate) fn extract_operator(s: &str) -> (&str,&str){
    extract_while(|c| !c.is_ascii_alphanumeric() && c != ' ', s)
}

pub(crate) fn extract_whitespace(s: &str) -> (&str,&str){
    extract_while(|c| c==' ', s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_digit_none(){
        assert_eq!(extract_digit(""), ("",""));
    }
    
    #[test]
    fn extract_digit_one(){
        assert_eq!(extract_digit("5+7"), ("5","+7"));
    }
    
    #[test]
    fn extract_digit_several(){
        assert_eq!(extract_digit("53+7"), ("53","+7"));
    }
    
    #[test]
    fn extract_digit_only(){
        assert_eq!(extract_digit("557"), ("557",""));
    }
    
    #[test]
    fn extract_digit_not_digit(){
        assert_eq!(extract_digit("+7"), ("","+7"));
    }
    
    #[test]
    fn extract_operator_none(){
        assert_eq!(extract_operator(""), ("",""));
    }
    
    #[test]
    fn extract_operator_one(){
        assert_eq!(extract_operator("+7"), ("+","7"));
    }
    
    #[test]
    fn extract_operator_several(){
        assert_eq!(extract_operator("//7"), ("//","7"));
    }
    
    #[test]
    fn extract_operator_only(){
        assert_eq!(extract_operator("*"), ("*",""));
    }
    
    #[test]
    fn extract_operator_not_digit(){
        assert_eq!(extract_operator("7"), ("","7"));
    }

    #[test]
    fn extract_whitespace_none(){
        assert_eq!(extract_whitespace(""), ("",""));
    }
    
    #[test]
    fn extract_whitespace_some(){
        assert_eq!(extract_whitespace("  7"), ("  ","7"));
    }
}