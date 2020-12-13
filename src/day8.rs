use std::convert::TryInto;
use std::iter::FromIterator;
use std::str::FromStr;

use anyhow::bail;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Instruction {
    NoOperation(i32),
    Accumulate(i32),
    Jump(i32),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction_str, count_str) = {
            let mut chunks = s.split_whitespace();

            (
                chunks
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("Could not parse instruction chunk"))?,
                chunks
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("Could not parse count chunk"))?,
            )
        };

        let value: i32 = count_str.parse()?;

        let instruction = match instruction_str {
            "nop" => Self::NoOperation(value),
            "acc" => Self::Accumulate(value),
            "jmp" => Self::Jump(value),
            _ => bail!("Invalid instruction: {}", instruction_str),
        };

        Ok(instruction)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CompletionState {
    OutOfBounds,
    Loop,
    Finished,
}

#[derive(Clone, Debug, PartialEq)]
struct Program {
    instructions: Vec<Instruction>,
    accumulator: i32,
    counter: i32,
}

impl Program {
    fn accumulator(&self) -> i32 {
        self.accumulator
    }

    fn counter(&self) -> i32 {
        self.counter
    }

    fn len(&self) -> usize {
        self.instructions.len()
    }

    fn step(&mut self) -> Option<i32> {
        let ret = self.counter;

        let index: usize = self.counter.try_into().ok()?;

        let instruction = self.instructions.get(index)?;

        self.counter += match instruction {
            Instruction::NoOperation(_) => 1,
            Instruction::Accumulate(count) => {
                self.accumulator += count;
                1
            }
            Instruction::Jump(offset) => *offset,
        };

        Some(ret)
    }

    fn run(&mut self) -> CompletionState {
        use std::collections::HashSet;

        let mut executed_instructions = HashSet::new();

        loop {
            let instruction = self.counter();

            if executed_instructions.contains(&instruction) {
                return CompletionState::Loop;
            }

            if self.step().is_none() {
                return if self.counter() == self.instructions.len() as i32 {
                    CompletionState::Finished
                } else {
                    CompletionState::OutOfBounds
                };
            }

            executed_instructions.insert(instruction);
        }
    }

    fn with_flipped_instruction(&self, index: usize) -> Option<Self> {
        let instruction = self.instructions.get(index)?;

        let new_instruction = match instruction {
            Instruction::NoOperation(n) => Instruction::Jump(*n),
            Instruction::Jump(n) => Instruction::NoOperation(*n),
            _ => return None,
        };

        let mut new_instructions = self.instructions.clone();
        new_instructions[index] = new_instruction;

        Some(new_instructions.into_iter().collect())
    }
}

impl FromIterator<Instruction> for Program {
    fn from_iter<I: IntoIterator<Item = Instruction>>(iter: I) -> Self {
        Self {
            instructions: iter.into_iter().collect(),
            accumulator: 0,
            counter: 0,
        }
    }
}

impl FromStr for Program {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let program: Self = s
            .lines()
            .map(|line| Instruction::from_str(line))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .collect();

        Ok(program)
    }
}

pub fn part1(raw_input: &str) -> anyhow::Result<i32> {
    let mut program: Program = raw_input.parse()?;

    if program.run() != CompletionState::Loop {
        bail!("Loop not found in program");
    }

    Ok(program.accumulator())
}

pub fn part2(raw_input: &str) -> anyhow::Result<i32> {
    let program: Program = raw_input.parse()?;

    let modified_programs = (0..program.len())
        .filter_map(|instruction_index| program.with_flipped_instruction(instruction_index));

    for mut program in modified_programs {
        if program.run() == CompletionState::Finished {
            return Ok(program.accumulator());
        }
    }

    bail!("No correct programs found");
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            Instruction::from_str("nop +0").unwrap(),
            Instruction::NoOperation(0)
        );
        assert_eq!(
            Instruction::from_str("acc +1").unwrap(),
            Instruction::Accumulate(1)
        );
        assert_eq!(
            Instruction::from_str("acc -99").unwrap(),
            Instruction::Accumulate(-99)
        );
        assert_eq!(
            Instruction::from_str("jmp +4").unwrap(),
            Instruction::Jump(4)
        );
    }

    #[test]
    fn test_collect_program() {
        assert_eq!(
            [Instruction::NoOperation(0)]
                .iter()
                .copied()
                .collect::<Program>(),
            Program {
                instructions: vec![Instruction::NoOperation(0)],
                accumulator: 0,
                counter: 0
            }
        );
    }

    #[test]
    fn test_parse_program() {
        let input = indoc! {"
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6
        "};

        use Instruction::{Accumulate, Jump, NoOperation};

        assert_eq!(
            input.parse::<Program>().unwrap(),
            Program {
                instructions: vec![
                    NoOperation(0),
                    Accumulate(1),
                    Jump(4),
                    Accumulate(3),
                    Jump(-3),
                    Accumulate(-99),
                    Accumulate(1),
                    Jump(-4),
                    Accumulate(6),
                ],
                accumulator: 0,
                counter: 0
            }
        );
    }

    fn sample_program() -> Program {
        let input = indoc! {"
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6
        "};

        input.parse().unwrap()
    }

    #[test]
    fn test_step_program() {
        let mut program = sample_program();

        assert_eq!(program.step(), Some(0));
        assert_eq!(program.step(), Some(1));
        assert_eq!(program.step(), Some(2));
        assert_eq!(program.step(), Some(6));
        assert_eq!(program.step(), Some(7));
        assert_eq!(program.step(), Some(3));
        assert_eq!(program.step(), Some(4));
        assert_eq!(program.step(), Some(1));
    }

    #[test]
    fn test_find_loop() {
        let mut program = sample_program();

        assert_eq!(program.run(), CompletionState::Loop);

        assert_eq!(program.accumulator(), 5);
    }
}
