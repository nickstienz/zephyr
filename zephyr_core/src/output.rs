use crate::instructions::{self, Instructions};

pub struct Output {
    output: String,
    col: usize,
}

impl Output {
    pub fn new() -> Output {
        Output {
            output: String::new(),
            col: 0,
        }
    }

    pub fn push_instruction(&mut self, instruction: Instructions, arguments: Vec<usize>) {
        // What the actual hell am I doing here with this shitty project. This needs a major rework.
        self.col += 1;
        let instruction = format!("{:02x}", instruction as u8);
        let arguments = arguments
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let output = format!("{}{}{}", instruction, arguments, "00");
        if self.col == 20 {
            self.output.push('\n');
        }
        self.output.push_str(&format!("{}", output));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let mut output = Output::new();
        output.push_instruction(Instructions::Start, vec![]);
        assert_eq!(output.output, "0100")
    }
}
