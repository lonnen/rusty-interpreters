use std::result;

pub struct VM { 
    program: Vec<Opcode>,
    stack: Vec<i64>,
}

pub enum Opcode {
    Push(i64),
    Add,
    Sub,
    Div,
    Mul,
    Done,
}

#[derive(Debug)]
pub enum ProgramError {
    StackUnderflow,
}

type Result<T> = result::Result<T, ProgramError>;

macro_rules! make_opcode {
    ($vm:expr, $opcode:tt) => {{
        if let Some(a) = $vm.stack.pop() {
            if let Some(b) = $vm.stack.pop() {
                $vm.stack.push(b $opcode a);
                None
            } else { Some(ProgramError::StackUnderflow) }
        } else { Some(ProgramError::StackUnderflow) }
    }
}}

pub fn interpret(program: Vec<Opcode>) -> Result<i64> {
    let mut vm = VM {
        program: program,
        stack: Vec::new(),
    };

    for i in vm.program {
        if let Some(err) = match i {
            Opcode::Push(i) => {
                vm.stack.push(i);
                None
            },
            Opcode::Add => make_opcode!(vm, +),
            Opcode::Sub => make_opcode!(vm, -),
            Opcode::Div => make_opcode!(vm, /),
            Opcode::Mul => make_opcode!(vm, *),
            Opcode::Done => break
        } {
            return Err(err);
        }
    }

    if let Some(res) = vm.stack.pop() {
        Ok(res)
    } else {
        Err(ProgramError::StackUnderflow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inc() {
        use Opcode::*;
        assert_eq!(interpret(vec![Push(2)]).unwrap(), 2);
        assert_eq!(interpret(vec![Push(3), Push(2), Sub, Done]).unwrap(), 1);
        assert_eq!(interpret(vec![Push(2), Push(3), Mul]).unwrap(), 6);
        assert_eq!(interpret(vec![Push(7), Push(1), Div, Done]).unwrap(), 7);
        assert_eq!(interpret(vec![Push(7), Push(5), Add, Push(3), Add, Push(3), Div, Done]).unwrap(), 5);
    }
}