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
        registers: [0; REGISTER_COUNT],
    };

    for op in vm.program {
        match op {
            Opcode::Load(source_0, value) => vm.registers[source_0] = value,
            Opcode::Add(source_0, source_1, destination) => {
                vm.registers[destination] = vm.registers[source_0] + vm.registers[source_1]
            },
            Opcode::Sub(source_0, source_1, destination) => {
                vm.registers[destination] = vm.registers[source_0] - vm.registers[source_1]
            },
            Opcode::Mul(source_0, source_1, destination) => {
                vm.registers[destination] = vm.registers[source_0] * vm.registers[source_1]
            },
            Opcode::Div(source_0, source_1, destination) => {
                if vm.registers[source_1] == 0 {
                    return Err(ProgramError::DivisionByZero);
                }
                vm.registers[destination] = vm.registers[source_0] / vm.registers[source_1]
            }
            Opcode::Done(source_0) => return Ok(vm.registers[source_0])
        }
    }

    Err(ProgramError::UnexpectedTermination)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ops() {
        use Opcode::*;

        // no Done instruction should error
        assert!(interpret(vec![Load(1, 100)]).is_err());
        assert!(interpret(vec![]).is_err());

        // registers are zeroed at initialization
        for reg in 0..REGISTER_COUNT {
            assert_eq!(interpret(vec![Done(reg)]).unwrap(), 0);
        }
        
        // load a value
        assert_eq!(interpret(vec![Load(0, 2), Done(0)]).unwrap(), 2);

        // addition is commutative 1 of 2
        assert_eq!(
            interpret(vec![
                Load(0, 2),
                Load(1, 3),
                Add(0, 1, 0),
                Done(0)
            ]).unwrap(),
            5
        );

        // addition is commutative 2 of 2
        assert_eq!(
            interpret(vec![
                Load(0, 3),
                Load(1, 2),
                Add(0, 1, 0),
                Done(0)
            ]).unwrap(),
            5
        );

        // addition should not modify memory
        assert_eq!(
            interpret(vec![
                Load(0, 2),
                Load(1, 2),
                Add(0, 1, 0),
                Done(1)
            ]).unwrap(),
            2
        );

        // no other memory should be impacted
        assert_eq!(
            interpret(vec![
                Load(0, 2),
                Load(1, 2),
                Add(0, 1, 0),
                Done(2)
            ]).unwrap(),
            0
        );


        // subtraction is not commutative 1 of 2
        assert_eq!(
            interpret(vec![
                Load(0, 2),
                Load(1, 3),
                Sub(0, 1, 0),
                Done(0)
            ]).unwrap(),
            -1
        );

        // subtraction is commutative 2 of 2
        assert_eq!(
            interpret(vec![
                Load(0, 3),
                Load(1, 2),
                Sub(0, 1, 0),
                Done(0)
            ]).unwrap(),
            1
        );

        // subtraction should not modify memory
        assert_eq!(
            interpret(vec![
                Load(0, 2),
                Load(1, 2),
                Sub(0, 1, 0),
                Done(1)
            ]).unwrap(),
            2
        );

        // no other memory should be impacted
        assert_eq!(
            interpret(vec![
                Load(0, 2),
                Load(1, 2),
                Sub(0, 1, 0),
                Done(2)
            ]).unwrap(),
            0
        );


        // multiplication is commutative 1 of 2
        assert_eq!(
            interpret(vec![
                Load(0, 2),
                Load(1, 3),
                Mul(0, 1, 0),
                Done(0)
            ]).unwrap(),
            6
        );

        // multiplication is commutative 2 of 2
        assert_eq!(
            interpret(vec![
                Load(0, 3),
                Load(1, 2),
                Mul(0, 1, 0),
                Done(0)
            ]).unwrap(),
            6
        );

        // addition should not modify memory
        assert_eq!(
            interpret(vec![
                Load(0, 2),
                Load(1, 2),
                Mul(0, 1, 0),
                Done(1)
            ]).unwrap(),
            2
        );

        // no other memory should be impacted
        assert_eq!(
            interpret(vec![
                Load(0, 2),
                Load(1, 2),
                Mul(0, 1, 0),
                Done(2)
            ]).unwrap(),
            0
        );


        // division is not commutative 1 of 2
        assert_eq!(
            interpret(vec![
                Load(0, 6),
                Load(1, 3),
                Div(0, 1, 0),
                Done(0)
            ]).unwrap(),
            2
        );

        // division is commutative 2 of 2
        assert_eq!(
            interpret(vec![
                Load(0, 3),
                Load(1, 6),
                Div(0, 1, 0),
                Done(0)
            ]).unwrap(),
            0
        );

        // division should not modify memory
        assert_eq!(
            interpret(vec![
                Load(0, 2),
                Load(1, 2),
                Div(0, 1, 0),
                Done(1)
            ]).unwrap(),
            2
        );

        // no other memory should be impacted
        assert_eq!(
            interpret(vec![
                Load(0, 2),
                Load(1, 2),
                Div(0, 1, 0),
                Done(2)
            ]).unwrap(),
            0
        );

        // integer division rounds down 1 of 2
        assert_eq!(
            interpret(vec![
                Load(0, 4),
                Load(1, 3),
                Div(0, 1, 0),
                Done(0)
            ]).unwrap(),
            1
        );
    
        // integer division rounds down 2 of 2
        assert_eq!(
            interpret(vec![
                Load(0, 3),
                Load(1, 4),
                Div(0, 1, 0),
                Done(0)
            ]).unwrap(),
            0
        );

        // divide zero by anything
        assert_eq!(
            interpret(vec![
                Load(0, 0),
                Load(1, 4),
                Div(0, 1, 0),
                Done(0)
            ]).unwrap(),
            0
        );
        
        // divide by zero
        assert!(
            interpret(vec![
                Load(0, 1),
                Load(1, 0),
                Div(0, 1, 0),
                Done(0)
            ]).is_err()
        );

        // access out of range
        //assert_eq!(interpret(vec![Done(REGISTER_COUNT + 1)]).unwrap(), 0);
        // unimplemented. this simply panics, now
    }
}