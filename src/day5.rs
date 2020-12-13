use std::convert::{TryFrom, TryInto as _};
use std::str::FromStr;
use std::collections::BTreeSet;

use anyhow::bail;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Fb {
    Forward,
    Back,
}

impl TryFrom<char> for Fb {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            'F' => Self::Forward,
            'B' => Self::Back,
            _ => bail!("Unexpected f/b character: {}", c),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Rl {
    Right,
    Left,
}

impl TryFrom<char> for Rl {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => bail!("Unexpected r/l character: {}", c),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Seat {
    row: i32,
    col: i32,
}

impl Seat {
    fn id(&self) -> i32 {
        (self.row * 8) + self.col
    }
}

#[derive(Clone, Debug, PartialEq)]
struct BoardingPass {
    fbs: [Fb; 7],
    rls: [Rl; 3],
}

impl FromStr for BoardingPass {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fbs = s
            .chars()
            .take(7)
            .map(Fb::try_from)
            .collect::<Result<Vec<_>, _>>()?
            .as_slice()
            .try_into()?;

        let rls = s
            .chars()
            .skip(7)
            .take(3)
            .map(Rl::try_from)
            .collect::<Result<Vec<_>, _>>()?
            .as_slice()
            .try_into()?;

        Ok(Self { fbs, rls })
    }
}

fn binary_partition<T: PartialEq>(values: &[T], length: usize, high_value: T) -> i32 {
    values
        .iter()
        .enumerate()
        .filter(|(_i, half)| **half == high_value)
        .map(|(i, _half)| {
            let exponent = length - 1 - i;
            let exponent = exponent.try_into().unwrap();
            (2 as i32).pow(exponent)
        })
        .sum()
}

impl BoardingPass {
    fn find_seat(&self) -> Seat {
        let row = binary_partition(&self.fbs, self.fbs.len(), Fb::Back);
        let col = binary_partition(&self.rls, self.rls.len(), Rl::Right);

        Seat { row, col }
    }
}

pub fn part1(raw_input: &str) -> anyhow::Result<i32> {
    let passes: Vec<_> = raw_input
        .lines()
        .map(|line| line.parse::<BoardingPass>())
        .collect::<Result<_, _>>()?;

    let max_id = passes
        .iter()
        .map(|pass| pass.find_seat().id())
        .max()
        .unwrap_or_default();

    Ok(max_id)
}

pub fn part2(raw_input: &str) -> anyhow::Result<i32> {
    let passes: Vec<_> = raw_input
        .lines()
        .map(|line| line.parse::<BoardingPass>())
        .collect::<Result<_, _>>()?;

    let list_ids: BTreeSet<i32> = passes
        .iter()
        .map(|pass| pass.find_seat().id())
        .collect();

    let all_ids: BTreeSet<i32> = (0..(128 * 8)).collect();

    let missing_ids: BTreeSet<_> = all_ids.difference(&list_ids).collect();

    for id in &missing_ids {
        if !missing_ids.contains(&(*id + 1)) && !missing_ids.contains(&(*id - 1)) {
            return Ok(**id)
        }
    }

    bail!("Id not found!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fb() {
        assert_eq!(Fb::try_from('F').unwrap(), Fb::Forward);
        assert_eq!(Fb::try_from('B').unwrap(), Fb::Back);
        assert!(Fb::try_from('C').is_err());
    }

    #[test]
    fn test_parse_rl() {
        assert_eq!(Rl::try_from('R').unwrap(), Rl::Right);
        assert_eq!(Rl::try_from('L').unwrap(), Rl::Left);
        assert!(Rl::try_from('C').is_err());
    }

    #[test]
    fn test_parse_boarding_pass() {
        use Fb::{Back, Forward};
        use Rl::{Left, Right};

        assert_eq!(
            BoardingPass::from_str("FBFBBFFRLR").unwrap(),
            BoardingPass {
                fbs: [Forward, Back, Forward, Back, Back, Forward, Forward],
                rls: [Right, Left, Right],
            }
        );
    }

    #[test]
    fn test_find_seat() {
        let pass = BoardingPass::from_str("FBFBBFFRLR").unwrap();
        assert_eq!(pass.find_seat(), Seat { row: 44, col: 5 });
    }

    #[test]
    fn test_seat_id() {
        assert_eq!(Seat { row: 44, col: 5 }.id(), 357);
    }
}
