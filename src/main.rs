use std::io;
use std::fmt;

#[derive(Debug)] // needed in tests
#[derive(PartialEq)] // needed in tests
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

fn read_operand(prompt: &'static str, mut reader: impl io::BufRead) -> i32 {
    loop {
        println!("{}", prompt);
        let mut op = String::new();
        reader.read_line(&mut op).expect("Errore di lettura operando");
        let parsed_op = op.trim().parse::<i32>();
        if parsed_op.is_ok() {
            break parsed_op.unwrap();
        } 
    }
}

fn read_operator(prompt: &'static str, mut reader: impl io::BufRead) -> Operator {
    loop {
        println!("{}", prompt);
        let mut op = String::new();
        reader.read_line(&mut op).expect("Errore di lettura operando");
        let op = op.chars().nth(0).expect("Errore di lettura del primo carattere");
        let parsed_op = Operator::from_char(op);
        if parsed_op.is_ok() {
            break parsed_op.unwrap();
        }
    }
}

fn main() {
    let op = read_operator("Quale operazione vuoi fare? [ + - x : ]", &mut io::stdin().lock());
    let op1 = read_operand("Primo operando?", &mut io::stdin().lock());
    let op2 = read_operand("Secondo operando?", &mut io::stdin().lock());

    let operation: Operation = Operation {
        op, operand1: op1, operand2: op2
    };

    let result = calculate(&operation);

    let formatted_result = format_result(&operation, result);

    println!("{}", formatted_result);
}

fn calculate(operation: &Operation) -> i32 {
    match operation.op {
        Operator::Plus => operation.operand1 + operation.operand2,
        Operator::Minus => operation.operand1 - operation.operand2,
        Operator::Times => operation.operand1 * operation.operand2,
        Operator::Divided => operation.operand1 / operation.operand2,
    }
}

fn format_result(operation: &Operation, result: i32) -> String {
    format!("Il risultato di {0} {1} {2} è {3}", operation.operand1, operation.op, operation.operand2, result)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reading_times_operator() {
        let input = "x\n".as_bytes();
        
        let operator = read_operator("Op?", input);
        assert_eq!(operator, Operator::Times)
    }
    
    #[test]
    fn reading_times_operator_multiple() {
        let input = "3\nt\nx\n".as_bytes();
        
        let operator = read_operator("Op?", input);
        assert_eq!(operator, Operator::Times)
    }

    #[test]
    fn reading_operand() {
        let input = "3\n".as_bytes();
        
        let operator = read_operand("Op?", input);
        assert_eq!(operator, 3)
    }

    #[test]
    fn reading_operand_multiple_times() {
        let input = "x\na\n3\n".as_bytes();
        
        let operator = read_operand("Op?", input);
        assert_eq!(operator, 3)
    }

    #[test]
    fn test_add() {
        let operation = Operation {op: Operator::Plus, operand1: 2, operand2: 2};
        let result = calculate(&operation);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_sub() {
        let operation = Operation {op: Operator::Minus, operand1: 5, operand2: 2};
        let result = calculate(&operation);
        assert_eq!(result, 3);
    }

}
