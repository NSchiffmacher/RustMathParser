mod expr;
mod tokenizer;

pub use expr::Expr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_addition() {
        let input = "3+4";
        let res: expr::T = 3+4;
        let expr = Expr::parse(input).unwrap();

        assert_eq!(expr.eval(), res);
    }

    #[test]
    fn basic_multiplication() {
        let input = "3*4";
        let res: expr::T = 3*4;
        let expr = Expr::parse(input).unwrap();

        assert_eq!(expr.eval(), res);
    }

    #[test]
    fn addition_and_left_mult() {
        let input = "3+4*5";
        let res: expr::T = 3+4*5;
        let expr = Expr::parse(input).unwrap();

        assert_eq!(expr.eval(), res);
    }

    #[test]
    fn addition_and_right_mult() {
        let input = "3*4+5";
        let res: expr::T = 3*4+5;
        let expr = Expr::parse(input).unwrap();

        assert_eq!(expr.eval(), res);
    }

    #[test]
    fn left_parentheses() {
        let input = "(3+4)*5";
        let res: expr::T = (3+4)*5;
        let expr = Expr::parse(input).unwrap();

        assert_eq!(expr.eval(), res);
    }

    #[test]
    fn right_parentheses() {
        let input = "3*(4+5)";
        let res: expr::T = 3*(4+5);
        let expr = Expr::parse(input).unwrap();

        assert_eq!(expr.eval(), res);
    }
}
