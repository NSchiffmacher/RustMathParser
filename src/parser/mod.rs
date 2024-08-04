mod expr;
mod preprocessor;
mod tokenizer;

pub use expr::Expr;
pub use expr::T;
pub use expr::OPERATORS_PRECEDENCE;

pub fn parse(input: &str) -> Result<expr::T, String> {
    let expr = Expr::parse(input)?;
    let result = expr.eval()?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_addition() {
        let input = "3+4";
        let res: expr::T = 3 + 4;
        let expr = Expr::parse(input).unwrap();

        assert_eq!(expr.eval().unwrap(), res);
    }

    #[test]
    fn basic_multiplication() {
        let input = "3*4";
        let res: expr::T = 3 * 4;
        let expr = Expr::parse(input).unwrap();

        assert_eq!(expr.eval().unwrap(), res);
    }

    #[test]
    fn addition_and_left_mult() {
        let input = "3+4*5";
        let res: expr::T = 3 + 4 * 5;
        let expr = Expr::parse(input).unwrap();

        assert_eq!(expr.eval().unwrap(), res);
    }

    #[test]
    fn addition_and_right_mult() {
        let input = "3*4+5";
        let res: expr::T = 3 * 4 + 5;
        let expr = Expr::parse(input).unwrap();

        assert_eq!(expr.eval().unwrap(), res);
    }

    #[test]
    fn left_parentheses() {
        let input = "(3+4)*5";
        let res: expr::T = (3 + 4) * 5;
        let expr = Expr::parse(input).unwrap();

        assert_eq!(expr.eval().unwrap(), res);
    }

    #[test]
    fn right_parentheses() {
        let input = "3*(4+5)";
        let res: expr::T = 3 * (4 + 5);
        let expr = Expr::parse(input).unwrap();

        assert_eq!(expr.eval().unwrap(), res);
    }
    
    #[test]
    fn crash_missmatch() {  
        let input = "3*(4+(5+)";
        assert!(Expr::parse(input).is_err());
    }

    #[test]
    fn complex1() {
        let input = "1 -1   + 2   - 2   +  4 - 4 +    6";
        let res: expr::T = 6;
        let expr = Expr::parse(input).unwrap();

        assert_eq!(expr.eval().unwrap(), res);
    }

    #[test]
    fn complex2() {
        let input = "2*3*4/8 -   5/2*4 +  6 + 0/3 ";
        let res: expr::T = 2 * 3 * 4 / 8 - 5 / 2 * 4 + 6 + 0 / 3;
        let expr = Expr::parse(input).unwrap();

        assert_eq!(expr.eval().unwrap(), res);
    }

    #[test]
    fn complex3() {
        let input: &str = "(2) + (17*2-30) * (5)+2 - (8/2)*4";
        let res: expr::T = 8;
        let expr = Expr::parse(input).unwrap();

        assert_eq!(expr.eval().unwrap(), res);
    }

    #[test]
    fn div_zero() {
        let input = "1/0";
        let expr = Expr::parse(input).unwrap();
        assert!(expr.eval().is_err());
    }

    #[test]
    fn negative_value() {
        let input = "-2";
        let expr = Expr::parse(input).unwrap();
        assert_eq!(expr.eval().unwrap(), -2);
    }

    #[test]
    fn neg_in_parentheses() {
        let input = "1 + (-2)";
        let expr = Expr::parse(input).unwrap();
        assert_eq!(expr.eval().unwrap(), -1);
    }

    #[test]
    fn basic_pow() {
        let input = "2^3";
        let res: expr::T = 8;
        let expr = Expr::parse(input).unwrap();

        assert_eq!(expr.eval().unwrap(), res);
    }

    #[test]
    fn two_pow() {
        let input = "2^3^2";
        let res: expr::T = 512;
        let expr = Expr::parse(input).unwrap();

        assert_eq!(expr.eval().unwrap(), res);
    }

    #[test]
    fn pow_parentheses() {
        let input = "2^(3+4)";
        let res: expr::T = 128;
        let expr = Expr::parse(input).unwrap();

        assert_eq!(expr.eval().unwrap(), res);
    }

    #[test]
    fn basic_mod() {
        let input = "7 % 3";
        let result = 1;
        let expr = Expr::parse(input).unwrap();

        assert_eq!(expr.eval().unwrap(), result);
    }

    #[test]
    fn helper_parse() {
        let input = "(2) + (17*2-30) * (5)+2 - (8/2)*4";
        let result: expr::T = 8;

        assert_eq!(parse(input).unwrap(), result);
    }
}
