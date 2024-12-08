use std::{clone, path::Path, result, thread};

#[derive(Copy, Clone)]
enum Operation {
    Add,
    Multiply,
    Concat,
}

impl Operation {
    pub fn iterator() -> impl Iterator<Item = Operation> {
        [Operation::Multiply, Operation::Add, Operation::Concat].iter().copied()
    }
}

#[derive(Copy, Clone)]
enum Value {
    Number(i128),
    Operation(Operation),
}

#[derive(Clone)]
struct Expression {
    values: Vec<Value>,
}

impl Expression {
    fn new() -> Expression {
        Expression {
            values: Vec::new(),
        }
    }

    fn evaluate(&self) -> i128 {
        let mut result = 0;
        let mut operation = Operation::Add;
        for value in &self.values {
            match value {
                Value::Number(n) => {
                    match operation {
                        Operation::Add => result += n,
                        Operation::Multiply => result *= n,
                        Operation::Concat => {
                            let result_str = result.to_string();
                            let n_str = n.to_string();
                            result = format!("{}{}", result_str, n_str).parse::<i128>().unwrap();
                        }
                    }
                }
                Value::Operation(op) => {
                    operation = op.clone();
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression() {
        let mut expression = Expression::new();
        expression.values.push(Value::Number(1));
        expression.values.push(Value::Operation(Operation::Add));
        expression.values.push(Value::Number(2));
        expression.values.push(Value::Operation(Operation::Multiply));
        expression.values.push(Value::Number(3));
        assert_eq!(expression.evaluate(), 9);
    }

    #[test]
    fn test_expression2() {
        let mut expression = Expression::new();
        expression.values.push(Value::Number(1));
        expression.values.push(Value::Operation(Operation::Add));
        expression.values.push(Value::Number(2));
        expression.values.push(Value::Operation(Operation::Add));
        expression.values.push(Value::Number(3));
        assert_eq!(expression.evaluate(), 6);
    }

    #[test]
    fn test_expression3() {
        let mut expression = Expression {
            values: vec![
                Value::Number(10),
                Value::Operation(Operation::Multiply),
                Value::Number(19),
                Value::Operation(Operation::Add),
                Value::Number(3),
            ],
        };
        assert_eq!(expression.evaluate(), 193);
    }

    #[test]
    fn test_expression4() {
        let mut expression = Expression {
            values: vec![
                Value::Number(10),
                Value::Operation(Operation::Multiply),
                Value::Number(19),
                Value::Operation(Operation::Add),
            ],
        };
        assert_eq!(expression.evaluate(), 190);
    }

    #[test]
    fn test_expression_concat() {
        let mut expression = Expression {
            values: vec![
                Value::Number(8),
                Value::Operation(Operation::Concat),
                Value::Number(6),
            ],
        };
        assert_eq!(expression.evaluate(), 86);
    }

    #[test]
    fn test_expression5() {
        let mut expression = Expression {
            values: vec![
                Value::Number(6),
                Value::Operation(Operation::Multiply),
                Value::Number(8),
                Value::Operation(Operation::Concat),
                Value::Number(6),
                Value::Operation(Operation::Multiply),
                Value::Number(15),
            ],
        };
        assert_eq!(expression.evaluate(), 7290);
    }
}

fn solve(result: i128, expr: Expression, open: Vec<i128>) -> bool {
    if expr.evaluate() > result {
        return false;
    }

    if expr.evaluate() == result && open.len() == 0 {
        return true;
    }

    if open.len() == 0 {
        return false;
    }
    
    for op in Operation::iterator() {
        let mut new_expr = expr.clone();
        let mut open = open.clone();
        new_expr.values.push(Value::Number(open[0]));
        open.remove(0);
        new_expr.values.push(Value::Operation(op));
        if solve(result, new_expr, open) {
            return true;
        }
    }

    return false;
}

fn main() {
    let path = Path::new("input.txt");
    let contents = std::fs::read_to_string(path).unwrap();
    let lines = contents.lines();
    let mut sum = 0;
    let (tx, rx) = std::sync::mpsc::channel();
    for line in lines {
        let s1: Vec<&str> = line.split(": ").collect();
        let result = s1[0].parse::<i128>().unwrap();
        let inputs = s1[1].split(" ").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<i128>>();
        println!("{:?}", inputs);

        let tx = tx.clone();
        thread::spawn(move || {
            if solve(result, Expression::new(), inputs) {
                println!("Found solution for {}", result);
                tx.send(result).unwrap();
            } else {
                println!("No solution found for {}", result);
            }
        });
    }
    drop(tx);
    for r in rx {
        sum += r;
    }
    println!("Sum: {}", sum);
}
