use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Entry {
    range: (usize, usize),
    character: char,
    password: String,
}

fn split_components(s: &str) -> anyhow::Result<(&str, &str, &str)> {
    let mut parts = s.split_whitespace();

    Ok((
        parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Could not parse range"))?,
        parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Could not parse character"))?,
        parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Could not parse password"))?,
    ))
}

fn parse_range(s: &str) -> anyhow::Result<(usize, usize)> {
    let mut parts = s.split('-');

    let begin = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("Could not parse beginning of range"))?
        .parse::<usize>()?;

    let end = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("Could not parse end of range"))?
        .parse::<usize>()?;

    Ok((begin, end))
}

fn parse_character(s: &str) -> anyhow::Result<char> {
    s.chars()
        .nth(0)
        .ok_or_else(|| anyhow::anyhow!("Could not parse character"))
}

impl FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (range_str, character_str, password_str) = split_components(s)?;

        let range = parse_range(range_str)?;

        let character = parse_character(character_str)?;

        let password = String::from(password_str);

        Ok(Self {
            range,
            character,
            password,
        })
    }
}

impl Entry {
    fn is_valid_part1(&self) -> bool {
        let count = count_occurances(self.character, &self.password);

        let (start, end) = self.range;

        (start..=end).contains(&count)
    }

    fn is_valid_part2(&self) -> bool {
        let (first, second) = self.range;

        let count = self.password.char_indices().filter(|(i, c)| {
            // Adjust for values not being 0 indexed.
            let index = i + 1;

            if (index != first) && (index != second) {
                return false;
            }

            *c == self.character
        }).count();

        count == 1
    }
}

fn count_occurances(character: char, s: &str) -> usize {
    s.chars().filter(|&c| c == character).count()
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Entry>> {
    let entries = input
        .lines()
        .map(Entry::from_str)
        .collect::<Result<_, _>>()?;

    Ok(entries)
}

pub fn part1(raw_input: &str) -> anyhow::Result<usize> {
    let input = parse_input(raw_input)?;

    let num_valid = input.iter().filter(|entry| entry.is_valid_part1()).count();

    Ok(num_valid)
}

pub fn part2(raw_input: &str) -> anyhow::Result<usize> {
    let input = parse_input(raw_input)?;

    let num_valid = input.iter().filter(|entry| entry.is_valid_part2()).count();

    Ok(num_valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_parse_entry() {
        let entry = "5-12 c: abcdefg".parse::<Entry>();

        assert_eq!(
            entry.unwrap(),
            Entry {
                range: (5, 12),
                character: 'c',
                password: String::from("abcdefg"),
            }
        );
    }

    #[test]
    fn test_count_occurances() {
        assert_eq!(count_occurances('b', "abcdbbe"), 3);
    }

    #[test]
    fn test_entry_is_valid_part1() {
        let entry = Entry {
            range: (1, 3),
            character: 'a',
            password: String::from("abcde"),
        };

        assert!(entry.is_valid_part1());

        let entry = Entry {
            range: (1, 3),
            character: 'b',
            password: String::from("cdefg"),
        };

        assert!(!entry.is_valid_part1());

        let entry = Entry {
            range: (2, 9),
            character: 'c',
            password: String::from("ccccccccc"),
        };

        assert!(entry.is_valid_part1());
    }

    #[test]
    fn test_entry_is_valid_part2() {
        let entry = Entry {
            range: (1, 3),
            character: 'a',
            password: String::from("abcde"),
        };

        assert!(entry.is_valid_part2());

        let entry = Entry {
            range: (1, 3),
            character: 'b',
            password: String::from("cdefg"),
        };

        assert!(!entry.is_valid_part2());

        let entry = Entry {
            range: (2, 9),
            character: 'c',
            password: String::from("ccccccccc"),
        };

        assert!(!entry.is_valid_part2());
    }

    #[test]
    fn test_parse_input() {
        let input = indoc! {"
            1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc
        "};

        let expected = vec![
            Entry {
                range: (1, 3),
                character: 'a',
                password: String::from("abcde"),
            },
            Entry {
                range: (1, 3),
                character: 'b',
                password: String::from("cdefg"),
            },
            Entry {
                range: (2, 9),
                character: 'c',
                password: String::from("ccccccccc"),
            },
        ];

        assert_eq!(parse_input(input).unwrap(), expected);
    }
}
