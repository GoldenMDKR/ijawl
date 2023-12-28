


mod utils;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s : &str) -> Self{
        Self(s.parse().unwrap())
    }

}


#[derive(Debug, PartialEq)]
pub enum Op{
    Add,
    Sub,
    Mul,
    Div
}

impl Op{
    pub fn new(s: &str) -> Self{
        match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("not/bad operator"),
        }
    }
}



#[derive(Debug, PartialEq)]
pub enum Value {
    Number(Number),
    Expression(Box<Expr>)
}

impl Value {
    pub fn as_num(val : Self) -> Number{
        match val {
            Value::Number(num) => num,
            Value::Expression(expr) => Expr::evaluate(*expr)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Expr{
    pub lhs: Value,
    pub rhs: Value,
    pub op: Op,
}

impl Expr{
    pub fn new(s: &str) -> Self{
        let (_,s) = utils::extract_whitespace(s);
        let (l,s) = utils::extract_digit(s);

        let (_,s) = utils::extract_whitespace(s);
        let (op,s) = utils::extract_operator(s);
        
        let (_,s) = utils::extract_whitespace(s);
        let (r,_) = utils::extract_digit(s);

        Self { 
            lhs: Value::Number(Number::new(l)), 
            rhs: Value::Number(Number::new(r)), 
            op: Op::new(op) 
        } 
    }

    pub fn evaluate(expr : Self) -> Number{
        let lh = Value::as_num(expr.lhs).0;
        let rh = Value::as_num(expr.rhs).0;
        match expr.op {
            Op::Add => Number(lh + rh),
            Op::Sub => Number(lh - rh),
            Op::Mul => Number(lh * rh),
            Op::Div => Number(lh / rh),
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number(){
        assert_eq!(Number::new("123"),Number(123))
    }

    #[test]
    fn parse_add_op(){
        assert_eq!(Op::new("+"), Op::Add);
    }
    
    #[test]
    fn parse_sub_op(){
        assert_eq!(Op::new("-"), Op::Sub);
    }
    
    #[test]
    fn parse_mul_op(){
        assert_eq!(Op::new("*"), Op::Mul);
    }
    
    #[test]
    fn parse_div_op(){
        assert_eq!(Op::new("/"), Op::Div);
    }

    #[test]
    fn parse_one_plus_two(){
        assert_eq!(
            Expr::new("1 + 2"),
            Expr {
                lhs: Value::Number(Number(1)),
                rhs: Value::Number(Number(2)),
                op: Op::Add,
            }
        );
    }

    #[test]
    fn evaluate_simple_add(){
        
        let exp = Expr::new("1+2");
        assert_eq!(
            Number(3),
            Expr::evaluate(exp)
        );
    }
    
    #[test]
    fn evaluate_simple_sub(){
        
        let exp = Expr::new("3-1");
        assert_eq!(
            Number(2),
            Expr::evaluate(exp)
        );
    }
    
    #[test]
    fn evaluate_simple_mul(){
        
        let exp = Expr::new("3*2");
        assert_eq!(
            Number(6),
            Expr::evaluate(exp)
        );
    }
    
    #[test]
    fn evaluate_simple_div(){
        
        let exp = Expr::new("12/2");
        assert_eq!(
            Number(6),
            Expr::evaluate(exp)
        );
    }
}
