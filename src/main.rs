use std::env::args;

const MEMORY_SIZE: usize = 30000;

struct Brainfuck {
    tokens: Vec<char>,
    program_cursor: usize,
    loop_stack: Vec<usize>,
    output: String,
    input: String,
    input_cursor: usize,
    memory: [u8; MEMORY_SIZE],
    memory_cursor: u8,
}

impl Brainfuck {
    pub fn run(source: String, input: String) -> String {
        let mut vm = Brainfuck {
            tokens: source.chars().collect(),
            program_cursor: 0,
            loop_stack: vec![],
            input,
            input_cursor: 0,
            memory: [0; MEMORY_SIZE],
            memory_cursor: 0,
            output: "".into(),
        };
        vm.evaluate()
    }

    pub fn evaluate(&mut self) -> String {
        while self.program_cursor < self.tokens.len() {
            self.tick();
        }
        self.output.clone()
    }

    fn tick(&mut self) {
        match self.token() {
            ',' => self.read_byte_from_input(),
            '.' => self.append_output(),
            '>' => self.next_cell(),
            '<' => self.prev_cell(),
            '+' => self.increment(),
            '-' => self.decrement(),
            '[' => self.jmp_if_zero(),
            ']' => self.jmp_if_not_zero(),
            _ => {}
        };
        self.program_cursor += 1;
    }

    fn set_cell_value(&mut self, value: u8) {
        self.memory[self.memory_cursor as usize] = value;
    }

    fn cell_value(&self) -> u8 {
        self.memory[self.memory_cursor as usize]
    }

    fn token(&self) -> char {
        self.tokens[self.program_cursor]
    }

    fn jmp_if_not_zero(&mut self) {
        let p = self.loop_stack.last();
        if self.cell_value() != 0 {
            if let Some(c) = p {
                self.program_cursor = *c
            }
        }
    }

    fn jmp_if_zero(&mut self) {
        self.loop_stack.push(self.program_cursor);
        if self.cell_value() == 0 {
            while self.tokens[self.program_cursor] != ']' {
                self.program_cursor += 1;
            }
        }
    }

    fn decrement(&mut self) {
        self.set_cell_value(self.cell_value().wrapping_sub(1));
    }

    fn increment(&mut self) {
        self.set_cell_value(self.cell_value().wrapping_add(1))
    }

    fn prev_cell(&mut self) {
        self.memory_cursor = self.memory_cursor.wrapping_sub(1)
    }

    fn next_cell(&mut self) {
        self.memory_cursor = self.memory_cursor.wrapping_add(1)
    }

    fn append_output(&mut self) {
        self.output.push(self.cell_value() as char)
    }

    fn read_byte_from_input(&mut self) {
        let curr_char = self.input.chars().nth(self.input_cursor).unwrap();
        self.set_cell_value(curr_char as u8);
        self.input_cursor += 1;
    }
}

fn main() {
    let arguments: Vec<String> = args().collect();
    let mut input = String::new();
    if arguments.len() >= 2 {
        input = arguments[1].to_string();
    }

    let source = String::from("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.");
    let output = Brainfuck::run(source, input);
    println!("{}", output);
}
