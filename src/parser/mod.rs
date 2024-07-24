use std::{collections::HashMap, fmt::Debug};

pub type T = i32;


pub enum Expr {
    Litteral(T),
    Add(Box<Expr>, Box<Expr>),
    Prod(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Litteral(val) => write!(f, "Litteral({})", val),
            Expr::Add(left, right) => write!(f, "Add({:?}, {:?})", left, right),
            Expr::Prod(left, right) => write!(f, "Prod({:?}, {:?})", left, right),
            Expr::Sub(left, right) => write!(f, "Sub({:?}, {:?})", left, right),
        }
    }
}

impl Expr {
    pub fn eval(&self) -> T {
        match self {
            Expr::Litteral(val) => *val,
            Expr::Add(left, right) => left.eval() + right.eval(),
            Expr::Prod(left, right) => left.eval() * right.eval(),
            Expr::Sub(left, right) => left.eval() - right.eval(),
        }
    }

    pub fn parse(input: &str) -> Result<Expr, String> {
        if let Ok(value) = input.parse::<T>() {
            return Ok(Expr::Litteral(value));
        }

        let split_res = split(input, vec!['+', '*', '-']);

        if split_res.len() == 0 {
            return Err(format!("Impossible ? (Should be a parsed litteral ?): {}", input));
        }
        
        // Least priority
        if split_res.contains_key(&'+') || split_res.contains_key(&'-') {
            let (split_char, (left, right)) = find_with_smallest_index(input, &split_res, vec!['+', '-']);
            match split_char {
                '+' => {
                    return Ok(Expr::Add(Box::new(Expr::parse(left)?), Box::new(Expr::parse(right)?)));
                },
                '-' => {
                    return Ok(Expr::Sub(Box::new(Expr::parse(left)?), Box::new(Expr::parse(right)?)));
                },
                _ => unreachable!(),
            }
        }

        if split_res.contains_key(&'*') {
            let (left, right) = split_on(input, split_res[&'*']);
            let left_expr = Expr::parse(left)?;
            let right_expr = Expr::parse(right)?;
            return Ok(Expr::Prod(Box::new(left_expr), Box::new(right_expr)));
        }

        Err(format!("Invalid input: {}", input))
    }
}

fn split(input: &str, separators: Vec<char>) -> HashMap<char, usize> {
    let mut res = HashMap::new();

    for (i, c) in input.chars().enumerate() {
        if separators.contains(&c) && !res.contains_key(&c) {
            res.insert(c, i);
            if res.len() == separators.len() {
                break;
            }
        }
    }

    res
}

fn split_on(input: &str, index: usize) -> (&str, &str) {
    let left = &input[..index];
    let right = &input[index+1..];
    (left, right)
}

fn find_with_smallest_index<'a>(input: &'a str, map: &HashMap<char, usize>, mut keys: Vec<char>) -> (char, (&'a str, &'a str)) {
    keys.sort_by_key(|c| map.get(c).unwrap_or(&usize::MAX));
    let key = keys[0];
    (key, split_on(input, map[&key]))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = "12+4*74+1*12-4";
        let expr = Expr::parse(input).unwrap();

        println!("Parse result: {:?}", expr);
        println!("Parse eval  : {:?}\n\n", expr.eval());
        assert_eq!(expr.eval(), 12+4*74+1*12-4);
    }
}