use super::expr::OPERATORS_PRECEDENCE;

pub fn tokenize(input: &str) -> Result<Vec<String>, String> {
    let mut tokens = vec![];
    let mut it = input.chars().peekable();

    while let Some(c) = it.next() {
        match c {
            c if c.is_numeric() => {
                let mut token = vec![c];
                while let Some(cc) = it.next_if(|x| x.is_numeric()) {
                    token.push(cc);
                }
                tokens.push(token.into_iter().collect());
            }
            '(' | ')' => tokens.push(c.to_string()),
            c if OPERATORS_PRECEDENCE.contains_key(c.to_string().as_str()) => tokens.push(c.to_string()),
            ' ' => {}
            _ => return Err(format!("Unrecognized token {c}")),
        };
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = "12 + (1 * 7)";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens, vec!["12", "+", "(", "1", "*", "7", ")"]);
    }
}
