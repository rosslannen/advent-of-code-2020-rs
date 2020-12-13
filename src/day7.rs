use std::collections::{BTreeMap, BTreeSet};

#[derive(Default, Debug, PartialEq)]
struct Rules<'a> {
    inner: BTreeMap<&'a str, BTreeMap<&'a str, i32>>,
}

impl<'a> Rules<'a> {
    fn new() -> Self {
        Self::default()
    }

    fn add_rule(&mut self, s: &'a str) -> anyhow::Result<()> {
        let ret_err = || anyhow::anyhow!("Could not parse rule: {}", s);

        let mut spaces = s.match_indices(' ');

        spaces.next().ok_or_else(ret_err)?; // Adjective
        let (i, _) = spaces.next().ok_or_else(ret_err)?; // Color
        let bag = &s[..i];

        spaces.next().ok_or_else(ret_err)?; // "bags"
        let (i, _) = spaces.next().ok_or_else(ret_err)?; // "contain"

        let rest = &s[(i + 1)..];

        if rest == "no other bags." {
            self.inner.insert(bag, BTreeMap::new());
            return Ok(());
        }

        let mut bags = BTreeMap::new();

        for chunk in rest.split(", ") {
            let mut spaces = chunk.match_indices(' ');

            let (i, _) = spaces.next().ok_or_else(ret_err)?; // Quantity

            let count = chunk[..i].parse::<i32>()?;

            spaces.next().ok_or_else(ret_err)?; // Adjective
            let (j, _) = spaces.next().ok_or_else(ret_err)?; // Color

            let bag = &chunk[(i + 1)..j];

            bags.insert(bag, count);
        }

        self.inner.insert(bag, bags);

        Ok(())
    }

    fn direct_contains(&self, s: &str) -> impl Iterator<Item = &str> {
        let s = String::from(s);

        self.inner
            .iter()
            .filter(move |(_bag, bags)| bags.contains_key::<str>(&s))
            .map(|(bag, _bags)| *bag)
    }

    fn contains(&self, start_bag: &str) -> BTreeSet<&str> {
        use std::collections::VecDeque;

        let mut bags = BTreeSet::new();

        let mut queue = VecDeque::new();

        queue.push_back(start_bag);

        while let Some(current_bag) = queue.pop_front() {
            for bag in self.direct_contains(current_bag) {
                let inserted = bags.insert(bag);
                if inserted {
                    queue.push_back(bag);
                }
            }
        }

        bags
    }

    fn num_contained_by(&self, top_bag: &str) -> anyhow::Result<i32> {
        let mut total = 0;

        let mut bag_stack = Vec::new();

        bag_stack.push((top_bag, 1));

        while let Some((current_bag, count)) = bag_stack.pop() {
            let bags = self
                .inner
                .get(current_bag)
                .ok_or_else(|| anyhow::anyhow!("Could not find entry for {}", current_bag))?;

            for (bag, c) in bags {
                bag_stack.push((bag, *c * count))
            }

            total += count;
        }

        Ok(total - 1)
    }
}

pub fn part1(raw_input: &str) -> anyhow::Result<usize> {
    let rules = {
        let mut rules = Rules::new();

        for line in raw_input.lines() {
            rules.add_rule(line)?;
        }

        rules
    };

    let num_bags = rules.contains("shiny gold").len();

    Ok(num_bags)
}

pub fn part2(raw_input: &str) -> anyhow::Result<i32> {
    let rules = {
        let mut rules = Rules::new();

        for line in raw_input.lines() {
            rules.add_rule(line)?;
        }

        rules
    };

    let num_bags = rules.num_contained_by("shiny gold")?;

    Ok(num_bags)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! b_tree_set {
        () => {
            BTreeSet::new()
        };
        ($($x:expr),+ $(,)?) => {
            {
                let mut temp = BTreeSet::new();
                $( temp.insert($x); )*
                    temp
            }
        };
    }

    macro_rules! b_tree_map {
        () => {
            BTreeMap::new()
        };
        ($($k:expr => $v:expr),+ $(,)?) => {
            {
                let mut temp = BTreeMap::new();
                $(temp.insert($k, $v);)*
                    temp
            }
        };
    }

    fn sample_rules() -> Rules<'static> {
        let input = [
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ];

        let mut rules = Rules::new();

        for i in &input {
            rules.add_rule(i).unwrap();
        }

        rules
    }

    #[test]
    fn test_parse_rule() {
        let rules = sample_rules();

        assert_eq!(
            rules.inner,
            b_tree_map! {
                "light red" => b_tree_map! {"bright white" => 1, "muted yellow" => 2},
                "dark orange" => b_tree_map! {"bright white" => 3, "muted yellow" => 4},
                "bright white" => b_tree_map! {"shiny gold" => 1},
                "muted yellow" => b_tree_map! {"shiny gold" => 2, "faded blue" => 9},
                "shiny gold" => b_tree_map! {"dark olive" => 1, "vibrant plum" => 2},
                "dark olive" => b_tree_map! {"faded blue" => 3, "dotted black" => 4},
                "vibrant plum" => b_tree_map! {"faded blue" => 5, "dotted black" => 6},
                "faded blue" => BTreeMap::new(),
                "dotted black" => BTreeMap::new(),
            }
        );
    }

    #[test]
    fn test_direct_containment() {
        let rules = sample_rules();

        assert_eq!(
            rules
                .direct_contains("shiny gold")
                .collect::<BTreeSet<&str>>(),
            b_tree_set! {"bright white", "muted yellow"}
        );
    }

    #[test]
    fn test_all_contains() {
        let rules = sample_rules();

        assert_eq!(
            rules.contains("shiny gold"),
            b_tree_set! {"bright white", "muted yellow", "dark orange", "light red"}
        );
    }

    #[test]
    fn test_contained_by() {
        let rules = sample_rules();

        assert_eq!(rules.num_contained_by("shiny gold").unwrap(), 32);
    }
}
