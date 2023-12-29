use std::borrow::Borrow;

#[path ="utils.rs"] mod utils;

#[derive(Debug, PartialEq, Clone)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> Self {
        Self(s.parse().unwrap())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Neg
}

impl Op {
    pub fn new(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("not/bad operator"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(Number),
    ExpressionBinary(Box<ExprBinary>),
    ExpressionUnary(Box<ExprUnary>)
}

impl Value {
    pub fn as_num(val: Self) -> Number {
        match val {
            Value::Number(num) => num,
            Value::ExpressionBinary(expr) => ExprBinary::evaluate(*expr),
            Value::ExpressionUnary(expr)=> ExprUnary::evaluate(*expr)
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct ExprUnary {
    pub val : Value,
    pub op: Op
}

impl ExprUnary {
    pub fn evaluate(expr: Self) -> Number {
        match expr.op {
            Op::Neg => Number(-Value::as_num(expr.val).0),
            _ => unreachable!()
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct ExprBinary {
    pub lhs: Value,
    pub rhs: Value,
    pub op: Op,
}

impl ExprBinary {
    pub fn new(s: &str) -> Self {
        let (_, s) = utils::extract_whitespace(s);
        let (l, s) = utils::extract_digit(s);

        let (_, s) = utils::extract_whitespace(s);
        let (op, s) = utils::extract_operator(s);

        let (_, s) = utils::extract_whitespace(s);
        let (r, _) = utils::extract_digit(s);

        Self {
            lhs: Value::Number(Number::new(l)),
            rhs: Value::Number(Number::new(r)),
            op: Op::new(op),
        }
    }

    pub fn evaluate(expr: Self) -> Number {
        let lh = Value::as_num(expr.lhs).0;
        let rh = Value::as_num(expr.rhs).0;
        match expr.op {
            Op::Add => Number(lh + rh),
            Op::Sub => Number(lh - rh),
            Op::Mul => Number(lh * rh),
            Op::Div => Number(lh / rh),
            _ => unreachable!()
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenInfo {
    Value(Value),
    Op(Op),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_info: TokenInfo,
    pub priority: u16,
}

// calculate the priority value of the token in token_list
// lower priority are closer to the root of the tree representing the operation
// they are supposed to be at 0
pub fn calculate_priority(token_list: &mut Vec<Token>) {
    for idx in 0..token_list.len() {
        match &token_list[idx].token_info {
            TokenInfo::Value(_val) => (),
            TokenInfo::Op(op) => match op {
                Op::Neg => token_list[idx + 1].priority = token_list[idx].priority + 1,
                Op::Mul | Op::Div => {
                    token_list[idx + 1].priority += 1;
                    for i in 1..idx+1{
                        match &token_list[idx - i].token_info {
                            TokenInfo::Value(_val) => token_list[idx - i].priority += 1,
                            TokenInfo::Op(op) => match op {
                                Op::Add | Op::Sub => {
                                    break;
                                }
                                _ => token_list[idx - i].priority += 1,
                            },
                    
                        }
                    };
                }
                Op::Add | Op::Sub => {
                    for i in 0..idx {
                        token_list[i].priority += 1;
                    }
                    for i in idx + 1..token_list.len() {
                        match &token_list[i].token_info {
                            TokenInfo::Value(_val) => token_list[i].priority += 1,
                            TokenInfo::Op(op) => match op {
                                Op::Add | Op::Sub => {
                                    break;
                                }
                                _ => token_list[i].priority += 1,
                            },
                        };
                    }
                }
            },
        };
    }
}

// input an expression
// the associated token_list
pub fn build_token_list(mut s: &str) -> Vec<Token>{
    let mut token_list :Vec<Token> = vec![];
    let mut new :&str;
    while s.len() > 0 {
        (new, s) = utils::extract_next_token(s);
        if new.len() == 0 {break;}
        let n1 = new.chars().next().unwrap();
        match n1 {
            '-' => {
                    match token_list.last() {
                        None => token_list.push(Token { token_info: TokenInfo::Op(Op::Neg), priority: 0 }),
                        Some(tok) => match tok.token_info {
                            TokenInfo::Op(_) => token_list.push(Token { token_info: TokenInfo::Op(Op::Neg), priority: 0 }),  
                            _ => token_list.push(Token { token_info: TokenInfo::Op(Op::Sub), priority: 0 })
                        }
                }
            }
            '+' | '*' | '/' => token_list.push(Token { token_info: TokenInfo::Op(Op::new(new)), priority: 0 }),
            _ => token_list.push(Token { token_info: TokenInfo::Value(Value::Number(Number::new(new))), priority: 0 }) 
        }
    };   
    token_list 
}

pub fn build_expr_tree(token_list: &[Token])-> Value{
    // can't have an epmpty list
    if token_list.len() == 0 {
        panic!("can't have a empty token_list");
    }
    // find the root of the tree
    let mut min = token_list[0].priority;
    let mut min_tok = token_list[0].token_info.borrow();
    let mut min_idx = 0;
    for idx in 1..token_list.len(){
        if token_list[idx].priority < min {
            min_idx = idx;
            min_tok = token_list[idx].token_info.borrow();
            min = token_list[idx].priority;
        };
    }


    // build the tree
    match min_tok {
        TokenInfo::Op(op) => match op {
            Op::Neg => Value::ExpressionUnary(Box::new(ExprUnary{
                    val : build_expr_tree(&token_list[min_idx+1..]),
                    op : (*op).clone(),
                })),
            _ => Value::ExpressionBinary(Box::new(ExprBinary { 
                lhs: build_expr_tree(&token_list[..min_idx]), 
                rhs: build_expr_tree(&token_list[min_idx+1..]), 
                op : (*op).clone() }))
            
            
        },
        TokenInfo::Value(val) => (*val).clone()
            
    }
}
    

pub fn calculate(s:&str)-> i32 {
    let mut list = build_token_list(s);
    calculate_priority(&mut list);
    let expr = build_expr_tree(&list);
    let res = Value::as_num(expr);
    res.0
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Number(123))
    }

    #[test]
    fn parse_add_op() {
        assert_eq!(Op::new("+"), Op::Add);
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("-"), Op::Sub);
    }

    #[test]
    fn parse_mul_op() {
        assert_eq!(Op::new("*"), Op::Mul);
    }

    #[test]
    fn parse_div_op() {
        assert_eq!(Op::new("/"), Op::Div);
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            ExprBinary::new("1 + 2"),
            ExprBinary {
                lhs: Value::Number(Number(1)),
                rhs: Value::Number(Number(2)),
                op: Op::Add,
            }
        );
    }

    #[test]
    fn evaluate_binary_simple_add() {
        let exp = ExprBinary::new("1+2");
        assert_eq!(Number(3), ExprBinary::evaluate(exp));
    }

    #[test]
    fn evaluate_binary_simple_sub() {
        let exp = ExprBinary::new("3-1");
        assert_eq!(Number(2), ExprBinary::evaluate(exp));
    }

    #[test]
    fn evaluate_binary_simple_mul() {
        let exp = ExprBinary::new("3*2");
        assert_eq!(Number(6), ExprBinary::evaluate(exp));
    }

    #[test]
    fn evaluate_binary_simple_div() {
        let exp = ExprBinary::new("12/2");
        assert_eq!(Number(6), ExprBinary::evaluate(exp));
    }

    #[test]
    fn evaluate_binary_complex() {
        // exp = 12 * (3 + 5)
        let exp = ExprBinary {
            lhs: Value::Number(Number(12)),
            rhs: Value::ExpressionBinary(Box::new(ExprBinary {
                lhs: Value::Number(Number(3)),
                rhs: Value::Number(Number(5)),
                op: Op::Add,
            })),
            op: Op::Mul,
        };
        assert_eq!(Number(96), ExprBinary::evaluate(exp));
    }

    #[test]
    fn evaluate_unary(){
        let expr = ExprUnary {
            val : Value::Number(Number(3)),
            op: Op::Neg
        };
        assert_eq!(Number(-3), ExprUnary::evaluate(expr))
    }

    #[test]
    fn priority_test() {
        let mut tokens_prio = vec![];
        tokens_prio.push(Token {
            token_info: TokenInfo::Value(Value::Number(Number(3))),
            priority: 3,
        });
        tokens_prio.push(Token {
            token_info: TokenInfo::Op(Op::Mul),
            priority: 2,
        });
        tokens_prio.push(Token {
            token_info: TokenInfo::Op(Op::Neg),
            priority: 3,
        });
        tokens_prio.push(Token {
            token_info: TokenInfo::Value(Value::Number(Number(5))),
            priority: 4,
        });
        tokens_prio.push(Token {
            token_info: TokenInfo::Op(Op::Add),
            priority: 1,
        });
        tokens_prio.push(Token {
            token_info: TokenInfo::Value(Value::Number(Number(6))),
            priority: 2,
        });
        tokens_prio.push(Token {
            token_info: TokenInfo::Op(Op::Add),
            priority: 0,
        });
        tokens_prio.push(Token {
            token_info: TokenInfo::Op(Op::Neg),
            priority: 1,
        });
        tokens_prio.push(Token {
            token_info: TokenInfo::Value(Value::Number(Number(3))),
            priority: 2,
        });

        let mut tokens = vec![];
        tokens.push(Token {
            token_info: TokenInfo::Value(Value::Number(Number(3))),
            priority: 0,
        });
        tokens.push(Token {
            token_info: TokenInfo::Op(Op::Mul),
            priority: 0,
        });
        tokens.push(Token {
            token_info: TokenInfo::Op(Op::Neg),
            priority: 0,
        });
        tokens.push(Token {
            token_info: TokenInfo::Value(Value::Number(Number(5))),
            priority: 0,
        });
        tokens.push(Token {
            token_info: TokenInfo::Op(Op::Add),
            priority: 0,
        });
        tokens.push(Token {
            token_info: TokenInfo::Value(Value::Number(Number(6))),
            priority: 0,
        });
        tokens.push(Token {
            token_info: TokenInfo::Op(Op::Add),
            priority: 0,
        });
        tokens.push(Token {
            token_info: TokenInfo::Op(Op::Neg),
            priority: 0,
        });
        tokens.push(Token {
            token_info: TokenInfo::Value(Value::Number(Number(3))),
            priority: 0,
        });

        calculate_priority(&mut tokens);
        assert_eq!(tokens_prio, tokens);
    }

    #[test]
    fn token_list_test(){
        let tok_list = build_token_list("3*-5+6+-3");
        let mut tokens = vec![];
        tokens.push(Token {
            token_info: TokenInfo::Value(Value::Number(Number(3))),
            priority: 0,
        });
        tokens.push(Token {
            token_info: TokenInfo::Op(Op::Mul),
            priority: 0,
        });
        tokens.push(Token {
            token_info: TokenInfo::Op(Op::Neg),
            priority: 0,
        });
        tokens.push(Token {
            token_info: TokenInfo::Value(Value::Number(Number(5))),
            priority: 0,
        });
        tokens.push(Token {
            token_info: TokenInfo::Op(Op::Add),
            priority: 0,
        });
        tokens.push(Token {
            token_info: TokenInfo::Value(Value::Number(Number(6))),
            priority: 0,
        });
        tokens.push(Token {
            token_info: TokenInfo::Op(Op::Add),
            priority: 0,
        });
        tokens.push(Token {
            token_info: TokenInfo::Op(Op::Neg),
            priority: 0,
        });
        tokens.push(Token {
            token_info: TokenInfo::Value(Value::Number(Number(3))),
            priority: 0,
        });
        assert_eq!(tok_list,tokens);
    }

    #[test]
    fn build_tree_test(){
        let s = "4*5-6+-2";
        let mut list = build_token_list(s);
        calculate_priority(&mut list);
        let expr = build_expr_tree(&list);
        let res = Value::as_num(expr);
        assert_eq!(res, Number(12));
    }

    #[test]
    fn calculate_test(){
        assert_eq!(calculate("3 + 2"), 5);
        assert_eq!(calculate("-5 * 3 +-2 "), -17);
        assert_eq!(calculate("- 5 * 8 / 2* -3  + 5   "), -5*8/2*-3+5);
    }
}
