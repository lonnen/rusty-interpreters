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

pub fn vm_match_recur(program: &[Opcode], sample: &[char], mut instruction: isize, mut symbol: isize) -> bool {
    loop {
        // ensure the loop is still within bounds
        if symbol < 0 || (symbol as usize) > sample.len() || instruction < 0 || (instruction as usize) > program.len() {
            return false;
        }
        
        match program[instruction as usize] {
            Opcode::Char(c) => {
                instruction += 1;
                if c == sample[symbol as usize] {
                    instruction += 1;
                    symbol += 1;
                } else {
                    false;
                }
            },
            Opcode::Jump(i) => {
                instruction += i;
            },
            Opcode::Or(left, right) => {
                if vm_match_recur(program, sample, instruction + left, symbol) {
                    return true;
                }
                instruction += right;
            },
            Opcode::Match => return true,
        }
    }
}

pub fn vm_match(program: Vec<Opcode>, sample: &str) -> bool {
    let s: Vec<char> = sample.chars().collect();
    return vm_match_recur(&program, &s, 0, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ops() {
        use Opcode::*;
    }
}