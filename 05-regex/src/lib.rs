pub enum Opcode {
    // match a single char to an immediate argument from the string
    // and advance ip and cp, or abort
    Char(char),
    // do an absolute jump to an offset, may be negative
    Jump(isize),
    // try to match the left or the right, effectively performing a
    // d-f s  for all regex options
    Or(isize, isize),
    // stop the execution and report a successful match
    Match,
}

pub fn vm_match_recur(
    program: &[Opcode], mut instruction: isize, mut symbol: char) -> bool {
    loop {
        match program[instruction as usize] {
            Opcode::Char(c) => {
                instruction += 1;
                
            },
            Opcode::Jump(i) => {
                instruction += i;
            },
            Opcode::Or(left, right) => {
                if vm_match_recur(program, instruction + left, symbol) {
                    return true;
                }
                instruction += right;
            },
            Opcode::Match => return true,
        }
    }
}

pub fn vm_match(program: Vec<Opcode>, sample: &str) -> bool {
    return true; // stubbed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ops() {
        use Opcode::*;
    }
}