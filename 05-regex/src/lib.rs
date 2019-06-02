pub enum Opcode {
    // match a single char to an immediate argument from the string
    // and advance ip and cp, or abort
    Char,
    // jump to and match either left expression or the right one, or abort
    Or,
    // do an absolute jump to an offset in the immediate argument
    Jump,
    // stop the execution and report a successful match
    Match,
}

pub enum Result {
    Ok,
    Fail,
    Error,
}

pub fn vm_match_recur(bytecode: Vec<Opcode>, ip: u8, sp: char) -> bool {
    return true; // stubbed
}

pub fn vm_match(bytecode: Vec<Opcode>, sample: &str) -> bool {
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