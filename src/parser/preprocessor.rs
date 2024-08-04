pub fn preprocess_tokens(tokens: Vec<String>) -> Result<Vec<String>, String> {
    let mut preprocessed_tokens = vec![];
    let mut it = tokens.iter();

    while let Some(token) = it.next() {
        match token.as_str() {
            "-" if preprocessed_tokens.is_empty() => preprocessed_tokens.push("0".to_string()),
            "-" => {
                    let previous_token = preprocessed_tokens.last().unwrap();
                    if previous_token == "(" {
                        preprocessed_tokens.push("0".to_string());
                    }
                },
            _ => {},
        };
        preprocessed_tokens.push(token.clone());
    }

    Ok(preprocessed_tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_through() {
        let tokens = vec!["12", "+", "(", "1", "*", "7", ")"].iter().map(|x| x.to_string()).collect();
        let preprocessed_tokens = preprocess_tokens(tokens).unwrap();
        assert_eq!(preprocessed_tokens, vec!["12", "+", "(", "1", "*", "7", ")"]);
    }

    #[test]
    fn negative_value() {
        let tokens = vec!["-", "12"].iter().map(|x| x.to_string()).collect();
        let preprocessed_tokens = preprocess_tokens(tokens).unwrap();
        assert_eq!(preprocessed_tokens, vec!["0", "-", "12"]);
    }

    #[test]
    fn negative_in_parentheses() {
        let tokens = vec!["4", "(", "-", "12", ")"].iter().map(|x| x.to_string()).collect();
        let preprocessed_tokens = preprocess_tokens(tokens).unwrap();
        assert_eq!(preprocessed_tokens, vec!["4", "(", "0", "-", "12", ")"]);
    }
}