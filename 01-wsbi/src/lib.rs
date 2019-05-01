pub enum Opcode {
    Inc,
    Dec,
    Done
}

pub struct VM {
    program: Vec<Opcode>,
    accumulate: i64
}

pub fn interpret(program: Vec<Opcode>) -> Option<i64> {
    let mut vm = VM {
        program: program,
        accumulate: 0,
    };

    for i in vm.program {
        vm.accumulate = match i {
            Opcode::Inc => vm.accumulate + 1,
            Opcode::Dec => vm.accumulate - 1,
            Opcode::Done => break
        }
    }

    return Some(vm.accumulate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inc() {
        use Opcode::*;
        assert_eq!(
            interpret(vec![Inc]), // note, no `Done`
            Some(1)
        );
    }

    #[test]
    fn dec() {
        use Opcode::*;
        assert_eq!(
            interpret(vec![Dec, Done]),
            Some(-1)
        );
    }

    #[test]
    fn long_program() {
        use Opcode::*;
        assert_eq!(
            interpret(vec![Inc, Dec, Inc]),
            Some(1)
        );
    }

    #[test]
    fn instructions_past_done() {
        use Opcode::*;
        assert_eq!(
            interpret(vec![Inc, Dec, Inc, Done, Inc]),
            Some(1)
        );
    }
}