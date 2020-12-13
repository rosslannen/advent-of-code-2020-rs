use anyhow::bail;

fn parse_input(raw_input: &str) -> anyhow::Result<Vec<i32>> {
    let values = raw_input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(values)
}

fn find_two_sum_to_2020(values: &[i32]) -> anyhow::Result<(i32, i32)> {
    for (i, num1) in values.into_iter().enumerate() {
        for num2 in &values[(i + 1)..] {
            let sum = num1 + num2;

            if sum == 2020 {
                return Ok((*num1, *num2));
            }
        }
    }

    bail!("Could not find 2 numbers adding 2020!");
}

fn find_three_sum_to_2020(values: &[i32]) -> anyhow::Result<(i32, i32, i32)> {
    for (i, num1) in values.into_iter().enumerate() {
        for (j, num2) in values[(i + 1)..].into_iter().enumerate() {
            for num3 in &values[(i + j + 1)..] {
                let sum = num1 + num2 + num3;
                if sum == 2020 {
                    return Ok((*num1, *num2, *num3));
                }
            }
        }
    }

    bail!("Could not find 2 numbers adding 2020!");
}

pub fn part1(raw_input: &str) -> anyhow::Result<i32> {
    let values = parse_input(raw_input)?;

    let (num1, num2) = find_two_sum_to_2020(&values)?;

    let result = num1 * num2;

    Ok(result)
}

pub fn part2(raw_input: &str) -> anyhow::Result<i32> {
    let values = parse_input(raw_input)?;

    let (num1, num2, num3) = find_three_sum_to_2020(&values)?;

    let result = num1 * num2 * num3;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let input = indoc! {"
            12
            23
            34
         "};

        assert_eq!(parse_input(input).unwrap(), vec![12, 23, 34]);
    }

    #[test]
    fn test_find_two_sum_to_2020() {
        let values = [1721, 979, 366, 299, 675, 1456];

        assert_eq!(find_two_sum_to_2020(&values).unwrap(), (1721, 299));
    }

    #[test]
    fn test_find_three_sum_to_2020() {
        let values = [1721, 979, 366, 299, 675, 1456];

        assert_eq!(find_three_sum_to_2020(&values).unwrap(), (979, 366, 675));
    }
}
