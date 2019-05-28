use std::result;

const REGISTER_COUNT: usize = 16;

pub struct VM { 
    program: Vec<Opcode>,
    registers: [i64; REGISTER_COUNT],
}

pub enum Opcode {
    Load(usize, i64),
    Add(usize, usize, usize),
    Sub(usize, usize, usize),
    Div(usize, usize, usize),
    Mul(usize, usize, usize),
    Done(usize),
}

#[derive(Debug)]
pub enum ProgramError {
    DivisionByZero,
    UnexpectedTermination,
    // UnknownOpcode, // compiler ensures this can never happen
}

type Result<T> = result::Result<T, ProgramError>;

pub fn interpret(program: Vec<Opcode>) -> Result<i64> {
    let mut vm = VM {
        program,
        stack: [0; REGISTER_COUNT],
    };

    for op in vm.program {
        match op {
            Opcode::Load(source_0, imm) => program.registers[r0] = imm,
            Opcode::Add(source_0, source_1, destination) => {

            },
            Opcode::Sub(source_0, source_1, destination) => {

            },
            Opcode::Mul(source_0, source_1, destination) => {
                
            },
            Opcode::Div(source_0, source_1, destination) => {

            }
            Opcode::Done(source_0) => return Ok(vm.registers)
        }
        Err(ProgramError::UnexpectedTermination)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inc() {
        use Opcode::*;
        assert_eq!(interpret(vec![Load(0, 2), Done(0)].unwrap(), 2);
        assert_eq!(interpret(vec![Load(1,2), Load(2, 3), Mul(1, 2, 0), Done(0)]).unwrap(), 6);
        assert_eq!(
            interpret(vec![
                Load(1, 2),
                Load(2, 2),
                Mul(1, 2, 0),
                Load(1, 3),
                Mul(1, 0, 0),
                Load(1, 4),
                Mul(1, 0, 0),
                Done(0)
            ]).unwrap(),
            48
        );
    }
}