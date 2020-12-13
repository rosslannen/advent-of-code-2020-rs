use std::collections::BTreeSet;

fn sum_group_answers_any(group: &str) -> usize {
    group
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<BTreeSet<char>>()
        .len()
}

fn sum_all_groups_any(answers: &str) -> usize {
    answers.split("\n\n").map(sum_group_answers_any).sum()
}

pub fn part1(raw_input: &str) -> anyhow::Result<usize> {
    Ok(sum_all_groups_any(raw_input))
}

fn sum_group_answers_all(group: &str) -> usize {
    let all_answers = ('a'..='z').collect::<BTreeSet<char>>();

    group
        .lines()
        .fold(all_answers, |remaining_answers, line| {
            line.chars()
                .collect::<BTreeSet<char>>()
                .intersection(&remaining_answers)
                .cloned()
                .collect::<BTreeSet<char>>()
        })
        .len()
}

fn sum_all_groups_all(answers: &str) -> usize {
    answers.split("\n\n").map(sum_group_answers_all).sum()
}

pub fn part2(raw_input: &str) -> anyhow::Result<usize> {
    Ok(sum_all_groups_all(raw_input))
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_sum_group_answers_any() {
        let group = indoc! {"
            abcx
            abcy
            abcz
        "};

        assert_eq!(sum_group_answers_any(group), 6);
    }

    #[test]
    fn test_sum_all_groups_any() {
        let answers = indoc! {"
            abc

            a
            b
            c

            ab
            ac

            a
            a
            a
            a

            b
        "};

        assert_eq!(sum_all_groups_any(answers), 11);
    }

    #[test]
    fn test_sum_group_answers_all() {
        let group = indoc! {"
            abcx
            abcy
            abcz
        "};

        assert_eq!(sum_group_answers_all(group), 3);
    }

    #[test]
    fn test_sum_all_groups_all() {
        let answers = indoc! {"
            abc

            a
            b
            c

            ab
            ac

            a
            a
            a
            a

            b
        "};

        assert_eq!(sum_all_groups_all(answers), 6);
    }
}
