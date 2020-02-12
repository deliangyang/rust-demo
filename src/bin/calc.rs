
#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Left,
}

#[derive(Debug)]
pub enum Token {
    Operator(Operator),
    Operand(isize),
}

pub fn eval(tokens: &[Token]) -> Option<isize> {
    let mut stack = Vec::new();
    if tokens.len() == 0 {
        return None;
    }

    for i in tokens {
        match i {
            &Token::Operand(isize) => {
                stack.push(isize);
                println!("{:?}", stack)
            }
            &Token::Operator(ref operator) => {
                if stack.len() <= 1 {
                    return None;
                }
                let op1 = stack.pop().unwrap();
                let op2 = stack.pop().unwrap();
                match operator {
                    &Operator::Add => {
                        stack.push(op1 + op2)
                    }
                    &Operator::Sub => {
                        stack.push(op2 - op1)
                    }
                    &Operator::Mul => {
                        stack.push(op1 * op2)
                    }
                    _ => {

                    }
                }
            }
        }
    }
    if stack.len() == 1 {
        let result = stack.pop().unwrap();
        Some(result)
    } else {
        None
    }
}

fn main() {
    // 3 2 5 - 6 * +
    // 2 * 23 + 12 - 2 * (2 + 1)
    let a = "5 - 1 - 2 -3 - 2 * (6 + 3 -1)";
    let chars = a.chars();
    let mut a = vec![];
    let mut stack = vec![];

    for c in chars {
        println!("{}", c);
        match c {
            '0'..='9' => {
                a.push(Token::Operand((c as isize) - 48))
            }
            '*' => {
                stack.push(Token::Operator(Operator::Mul))
            }
            '-' => {
                if !stack.is_empty() {
                    loop {
                        if stack.is_empty() {
                            break;
                        }
                        let top = stack.last().unwrap();
                        match top {
                            Token::Operator(operator) => {
                               match operator {
                                   Operator::Mul => {
                                       stack.pop().unwrap();
                                       a.push(Token::Operator(Operator::Mul));
                                   }
                                   Operator::Add => {
                                       stack.pop().unwrap();
                                       a.push(Token::Operator(Operator::Add));
                                   }
                                   Operator::Sub => {
                                        stack.pop().unwrap();
                                        a.push(Token::Operator(Operator::Sub));
                                   }
                                   _ => {
                                       println!("break -");
                                       break;
                                   }
                               }
                            }
                            _ => {
                                println!("break -");
                                break;
                            }
                        }
                    }
                }
                stack.push(Token::Operator(Operator::Sub))
            }
            '+' => {
                println!("stack {:?}", stack);
                if !stack.is_empty() {
                    loop {
                        if stack.is_empty() {
                            break;
                        }
                        let top = stack.last().unwrap();
                        println!("last: {:?}", top);
                        match top {
                            Token::Operator(operator) => {
                                match operator {
                                    Operator::Mul => {
                                        stack.pop().unwrap();
                                        a.push(Token::Operator(Operator::Mul));
                                    }
                                    Operator::Sub => {
                                        stack.pop().unwrap();
                                        a.push(Token::Operator(Operator::Sub));
                                    }
                                    Operator::Add => {
                                        stack.pop().unwrap();
                                        a.push(Token::Operator(Operator::Add));
                                    }
                                    _ => {
                                        println!("break -");
                                        break;
                                    }
                                }
                            }
                            _ => {break;}
                        }
                    }
                }
                stack.push(Token::Operator(Operator::Add))
            }
            '(' => {
                stack.push(Token::Operator(Operator::Left))
            }
            ')' => {
                loop {
                    let op = stack.pop().unwrap();
                    match op {
                        Token::Operator(operator) => {
                            match operator {
                                Operator::Left => {
                                    break;
                                }
                                _ => {
                                    a.push(Token::Operator(operator));
                                }
                            }
                        }
                        _ => {break;}
                    }
                }
            }
            _ => {}
        }
    };

    loop {
        if stack.is_empty() {
            break;
        }
        let op = stack.pop().unwrap();
        match op {
            Token::Operator(operator) => {
                a.push(Token::Operator(operator));
            }
            _ => {},
        }

    }

    println!("{:?}", &a);
    let x = eval(&a);
    println!("{}{:?}", "Your result is :", x);
}
