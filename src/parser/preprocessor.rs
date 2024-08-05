use super::T;

pub fn preprocess_tokens(tokens: Vec<String>) -> Result<Vec<String>, String> {
    let mut preprocessed_tokens = vec![];
    let mut it = tokens.iter().peekable();

    while let Some(token) = it.next() {
        match token.as_str() {
            // Handle case ... number ( expr ) => ... number * ( expr )
            token
                if (token.parse::<T>().is_ok() || token == ")")
                    && it.peek() == Some(&&"(".to_string()) =>
            {
                preprocessed_tokens.push(token.to_string());
                preprocessed_tokens.push("*".to_string());
            }
            // Handle case ... ) number => ... ) * number
            ")" if it.peek().map(|x| x.parse::<T>().is_ok()).unwrap_or(false) => {
                preprocessed_tokens.push(")".to_string());
                preprocessed_tokens.push("*".to_string());
            }
            // Handle inputs like "-12" => "0-12"
            "-" if preprocessed_tokens.is_empty() => {
                preprocessed_tokens.push("0".to_string());
                preprocessed_tokens.push(token.clone());
            }
            // Handle inputs like "1+(-12)" => "1+(-12)"
            "-" => {
                let previous_token = preprocessed_tokens.last().unwrap();
                if previous_token == "(" {
                    preprocessed_tokens.push("0".to_string());
                }
                preprocessed_tokens.push(token.clone());
            }
            // General case
            _ => {
                preprocessed_tokens.push(token.clone());
            }
        };
    }

    Ok(preprocessed_tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_through() {
        let tokens = vec!["12", "+", "(", "1", "*", "7", ")"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let preprocessed_tokens = preprocess_tokens(tokens).unwrap();
        assert_eq!(
            preprocessed_tokens,
            vec!["12", "+", "(", "1", "*", "7", ")"]
        );
    }

    #[test]
    fn negative_value() {
        let tokens = vec!["-", "12"].iter().map(|x| x.to_string()).collect();
        let preprocessed_tokens = preprocess_tokens(tokens).unwrap();
        assert_eq!(preprocessed_tokens, vec!["0", "-", "12"]);
    }

    #[test]
    fn negative_in_parentheses() {
        let tokens = vec!["4", "+", "(", "-", "12", ")"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let preprocessed_tokens = preprocess_tokens(tokens).unwrap();
        assert_eq!(
            preprocessed_tokens,
            vec!["4", "+", "(", "0", "-", "12", ")"]
        );
    }

    #[test]
    fn pre_parentheses() {
        let tokens = vec!["3", "(", "12", ")"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let preprocess_tokens = preprocess_tokens(tokens).unwrap();
        assert_eq!(preprocess_tokens, vec!["3", "*", "(", "12", ")"]);
    }

    #[test]
    fn post_parentheses() {
        let tokens = vec!["(", "12", ")", "3"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let preprocess_tokens = preprocess_tokens(tokens).unwrap();
        assert_eq!(preprocess_tokens, vec!["(", "12", ")", "*", "3"]);
    }
}
