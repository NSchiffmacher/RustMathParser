use super::preprocessor::preprocess_tokens;
use super::tokenizer::tokenize;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::sync::LazyLock;

pub type T = i32;

pub static OPERATORS_PRECEDENCE: LazyLock<HashMap<String, usize>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("+".to_string(), 0);
    map.insert("-".to_string(), 0);
    map.insert("*".to_string(), 1);
    map.insert("/".to_string(), 1);
    map.insert("%".to_string(), 1);
    map.insert("^".to_string(), 2);
    map
});
pub static RIGHT_ASSOCIATIVE_OPERATORS: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let mut set = HashSet::new();
    set.insert("^".to_string());
    set
});

pub enum Expr {
    Litteral(T),
    Add(Box<Expr>, Box<Expr>),
    Prod(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Expr::Litteral(val) => write!(f, "Litteral({})", val),
            Expr::Litteral(val) => write!(f, "{}", val),
            Expr::Add(left, right) => write!(f, "Add({:?}, {:?})", left, right),
            Expr::Prod(left, right) => write!(f, "Prod({:?}, {:?})", left, right),
            Expr::Div(left, right) => write!(f, "Div({:?}, {:?})", left, right),
            Expr::Sub(left, right) => write!(f, "Sub({:?}, {:?})", left, right),
            Expr::Pow(left, right) => write!(f, "Pow({:?}, {:?})", left, right),
            Expr::Mod(left, right) => write!(f, "Mod({:?}, {:?})", left, right),
        }
    }
}

impl Expr {
    pub fn eval(&self) -> Result<T, String> {
        match self {
            Expr::Litteral(val) => Ok(*val),
            Expr::Add(left, right) => Ok(left.eval()? + right.eval()?),
            Expr::Prod(left, right) => Ok(left.eval()? * right.eval()?),
            Expr::Sub(left, right) => Ok(left.eval()? - right.eval()?),
            Expr::Div(left, right) => {
                let left = left.eval()?;
                let right = right.eval()?;

                if right == T::from(0) {
                    return Err(format!("Cannot divide {} by zero", left));
                }

                Ok(left / right)
            }
            Expr::Pow(left, right) => {
                let left = left.eval()?;
                let right = right.eval()?;

                if left == T::from(0) && right == T::from(0) {
                    return Err("0^0 is undefined".to_string());
                }

                if right.is_negative() {
                    return Err("Cannot raise to a negative power".to_string());
                }

                Ok(left.pow(right as u32))
            }
            Expr::Mod(left, right) => {
                let left = left.eval()?;
                let right = right.eval()?;

                if right == T::from(0) {
                    return Err(format!("Cannot divide {} by zero", left));
                }

                Ok(left % right)
            }
        }
    }

    fn build_next_expr(
        operator: &str,
        expressions_queue: &mut VecDeque<Expr>,
    ) -> Result<(), String> {
        // For now we only have operators with two arguments
        if expressions_queue.len() < 2 {
            return Err("Missmatched operator".to_string());
        }

        let right = Box::new(expressions_queue.pop_back().unwrap());
        let left = Box::new(expressions_queue.pop_back().unwrap());

        let expr = match operator {
            "+" => Expr::Add(left, right),
            "-" => Expr::Sub(left, right),
            "*" => Expr::Prod(left, right),
            "/" => Expr::Div(left, right),
            "%" => Expr::Mod(left, right),
            "^" => Expr::Pow(left, right),
            _ => unreachable!(),
        };

        expressions_queue.push_back(expr);
        Ok(())
    }

    pub fn parse(input: &str) -> Result<Expr, String> {
        let mut operators_queue: VecDeque<String> = VecDeque::new();
        let mut expressions_queue: VecDeque<Expr> = VecDeque::new();

        let raw_tokens = tokenize(input)?;
        let tokens = preprocess_tokens(raw_tokens)?;
        for token in tokens {
            if let Ok(value) = token.parse::<T>() {
                expressions_queue.push_back(Expr::Litteral(value));
            } else if OPERATORS_PRECEDENCE.contains_key(&token) {
                while let Some(op2) = operators_queue.pop_back() {
                    if op2 != "("
                        && (OPERATORS_PRECEDENCE[&op2] > OPERATORS_PRECEDENCE[&token]
                            || (OPERATORS_PRECEDENCE[&op2] == OPERATORS_PRECEDENCE[&token]
                                && !RIGHT_ASSOCIATIVE_OPERATORS.contains(&token)))
                    {
                        Expr::build_next_expr(&op2, &mut expressions_queue)?;
                    } else {
                        operators_queue.push_back(op2);
                        break;
                    }
                }

                operators_queue.push_back(token);
            } else if token == "(" {
                operators_queue.push_back(token);
            } else if token == ")" {
                loop {
                    if operators_queue.is_empty() {
                        return Err("Mismatched parentheses".to_string());
                    }

                    let op = operators_queue.pop_back().unwrap();
                    if op == "(" {
                        break;
                    }

                    Expr::build_next_expr(&op, &mut expressions_queue)?;
                }
            }
        }

        while let Some(op) = operators_queue.pop_back() {
            if op == "(" {
                return Err("Mismatched parentheses".to_string());
            }

            Expr::build_next_expr(&op, &mut expressions_queue)?;
        }

        if expressions_queue.len() > 1 {
            return Err(format!(
                "Unable to parse {input}. Too many expressions remaining after parsing."
            ));
        }

        match expressions_queue.pop_back() {
            Some(expr) => Ok(expr),
            None => Err(format!("Unable to parse {input}")),
        }
    }
}
