use std::io;
use std::fmt;

enum Operator {
    Plus,
    Minus,
    Times,
    Divided,
}

impl Operator {
    fn from_char(c: char) -> Result<Operator, &'static str> {
        let chosen = match c {
            '+' => Operator::Plus,
            '-' => Operator::Minus,
            'x' => Operator::Times,
            ':' => Operator::Divided,
            _ => return Err("Operatore non riconosciuto")
        };

        Ok(chosen)
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = match &self {
            Operator::Plus => '+',
            Operator::Minus => '-',
            Operator::Times => 'x',
            Operator::Divided => ':',
        };
        write!(f, "{}", sign)
    }
}

struct Operation {
    op: Operator,
    operand1: i32,
    operand2: i32,
}

fn main() {

    let op = loop {
        println!("Quale operazione vuoi fare? [ + - x : ]");

        let mut op = String::new();
        io::stdin().read_line(&mut op).expect("Errore di lettura della prima linea");
        let op = op.chars().nth(0).expect("Errore di lettura del primo carattere");
        let parsed_op = Operator::from_char(op);
        if parsed_op.is_ok() {
            break parsed_op.unwrap();
        }
    };

    let op1 = loop {
        let mut op1 = String::new();
        println!("Primo operando?");
        io::stdin().read_line(&mut op1).expect("Errore di lettura primo operando");
        //     let op1 = op1.trim().parse().expect("Errore di interpretazione operatore 1"); 
        let parsed_op1 = op1.trim().parse::<i32>();
        if parsed_op1.is_ok() {
            break parsed_op1.unwrap();
        }
    };

    let op2 = loop {
        let mut op2 = String::new();
        println!("Secondo operando?");
        io::stdin().read_line(&mut op2).expect("Errore di lettura secondo operando");
        let parsed_op2 = op2.trim().parse::<i32>();
        if parsed_op2.is_ok() {
            break parsed_op2.unwrap();
        } 
    };

    let operation: Operation = Operation {
        op, operand1: op1, operand2: op2
    };

    let result = match operation.op {
        Operator::Plus => operation.operand1 + operation.operand2,
        Operator::Minus => operation.operand1 - operation.operand2,
        Operator::Times => operation.operand1 * operation.operand2,
        Operator::Divided => operation.operand1 / operation.operand2,
    };

    println!("{}", get_result_for_operation(&operation, result));
}

fn get_result_for_operation(operation: &Operation, result: i32) -> String {
    format!("Il risultato di {0} {1} {2} è {3}", operation.operand1, operation.op, operation.operand2, result)
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        // todo!();
        assert!(2 + 2 == 4);
        assert_eq!(2+2, 4);
        let operation = Operation {op: Operator::Plus, operand1: 2, operand2: 2};
        let result = get_result_for_operation(&operation, 4);
        assert_eq!(result, "Il risultato di 2 + 2 è 4");
    }
}
