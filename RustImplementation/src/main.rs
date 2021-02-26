use std::{cell::RefCell, collections::HashMap};

pub struct RPNCalculator<'a> {
    stack: RefCell<Vec<f64>>,
    operations: RefCell<HashMap<&'a str, Box<fn(f64, f64) -> f64>>>,
}

impl Default for RPNCalculator<'_> {
    fn default() -> Self {
        Self {
            stack: RefCell::new(Vec::new()),
            operations: RefCell::new(
                        [("+", Box::new((|a, b| a + b) as fn(f64, f64) -> f64)),
                         ("-", Box::new(|a, b| a - b)),
                         ("/", Box::new(|a, b| a / b)),
                         ("*", Box::new(|a, b| a * b))].iter().cloned().collect()),
        }
    }
}

impl RPNCalculator<'_> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn execute_line(&self, line: &str) {
        for instruction in line.split(" ") {
            self.execute(instruction);
        }
    }
    pub fn execute(&self, instruction: &str) {
        let mut stack = self.stack.borrow_mut();
        if self.operations.borrow().contains_key(instruction) {
            let (b, a) = (stack.pop().expect("Stack underflow"), stack.pop().expect("Stack underflow"));
            stack.push(self.operations.borrow_mut()[instruction](a, b));
        } else {
            stack.push(instruction.parse::<f64>().expect(&format!("Invalid instruction: {}", instruction)));
        }
    }
    pub fn get_last(&self) -> f64 {
        *self.stack.borrow().last().expect("Stack underflow")
    }
    pub fn get_dump(&self) -> String {
        format!("{:?}", self.stack.borrow())
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).expect("Failed to read a line");

    let calculator = RPNCalculator::new();
    calculator.execute_line(line.trim());

    println!("Last value on the stack: {}", calculator.get_last());
    println!("Stack dump: {}", calculator.get_dump());
}
