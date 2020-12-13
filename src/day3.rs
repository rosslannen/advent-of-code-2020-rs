use std::convert::{TryFrom, TryInto as _};
use std::str::FromStr;

use anyhow::bail;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Square {
    Open,
    Tree,
}

impl TryFrom<char> for Square {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let square = match c {
            '.' => Self::Open,
            '#' => Self::Tree,
            invalid => bail!("Invalid square character: {}", invalid),
        };

        Ok(square)
    }
}

#[derive(PartialEq, Debug)]
struct TreeMap {
    squares: Vec<Square>,
    width: usize,
    height: usize,
}

impl TreeMap {
    fn get(&self, row: usize, mut col: usize) -> Option<Square> {
        // Adjust the column for the infinite width of the forest.
        while col >= self.width {
            col -= self.width;
        }

        let index = col + (row * self.width);

        self.squares.get(index).copied()
    }

    fn count_trees_hit(&self, right_step: usize, down_step: usize) -> usize {
        let mut row = down_step;
        let mut col = right_step;

        let mut trees_hit = 0;

        while let Some(square) = self.get(row, col) {
            if square == Square::Tree {
                trees_hit += 1;
            }

            col += right_step;
            row += down_step;
        }

        trees_hit
    }
}

#[derive(Default)]
struct TreeMapBuilder {
    squares: Vec<Square>,
    width: Option<usize>,
    height: usize,
    valid: bool,
}

impl TreeMapBuilder {
    fn new() -> Self {
        Self {
            valid: true,
            ..Default::default()
        }
    }

    fn row(mut self, row: &[Square]) -> Self {
        if !self.valid {
            return self;
        }

        let row_width = row.len();

        let width = self.width.get_or_insert(row_width);

        if row_width != *width {
            self.valid = false;

            return self;
        }

        self.squares.extend_from_slice(row);

        self.height += 1;

        self
    }

    fn build(self) -> anyhow::Result<TreeMap> {
        if !self.valid {
            bail!("Invalid tree map");
        }

        let tree_map = TreeMap {
            squares: self.squares,
            width: self.width.unwrap_or_default(),
            height: self.height,
        };

        Ok(tree_map)
    }
}

impl TreeMap {
    fn builder() -> TreeMapBuilder {
        TreeMapBuilder::new()
    }
}

impl FromStr for TreeMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut builder = TreeMap::builder();

        let rows = s.lines().map(|line| {
            line.chars()
                .map(|c| c.try_into())
                .collect::<Result<Vec<_>, _>>()
        });

        for row in rows {
            let row = row?;

            builder = builder.row(&row);
        }

        let tree_map = builder.build()?;

        Ok(tree_map)
    }
}

pub fn part1(raw_input: &str) -> anyhow::Result<usize> {
    let tree_map: TreeMap = raw_input.parse()?;

    let trees_hit = tree_map.count_trees_hit(3, 1);

    Ok(trees_hit)
}

pub fn part2(raw_input: &str) -> anyhow::Result<usize> {
    let tree_map: TreeMap = raw_input.parse()?;

    let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let answer = slopes
        .iter()
        .map(|(r, d)| tree_map.count_trees_hit(*r, *d))
        .product();

    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_make_square() {
        assert_eq!(Square::try_from('.').unwrap(), Square::Open);
        assert_eq!(Square::try_from('#').unwrap(), Square::Tree);
        assert!(Square::try_from('&').is_err());
    }

    #[test]
    fn test_parse_tree_map() {
        let input = indoc! {"
            ..##
            #...
            .#..
        "};

        use Square::{Open, Tree};

        let expected = TreeMap {
            squares: vec![
                Open, Open, Tree, Tree, Tree, Open, Open, Open, Open, Tree, Open, Open,
            ],
            width: 4,
            height: 3,
        };

        assert_eq!(input.parse::<TreeMap>().unwrap(), expected);
    }

    #[test]
    fn test_build_tree_map() {
        use Square::{Open, Tree};

        let tree_map = TreeMap::builder()
            .row(&[Open, Open, Tree, Tree])
            .row(&[Tree, Open, Open, Open])
            .row(&[Open, Tree, Open, Open])
            .build()
            .unwrap();

        let expected = TreeMap {
            squares: vec![
                Open, Open, Tree, Tree, Tree, Open, Open, Open, Open, Tree, Open, Open,
            ],
            width: 4,
            height: 3,
        };

        assert_eq!(tree_map, expected);
    }

    #[test]
    fn test_index_tree_map() {
        let tree_map = indoc! {"
            ..##
            #...
            .#..
        "}
        .parse::<TreeMap>()
        .unwrap();

        assert_eq!(tree_map.get(1, 2), Some(Square::Open));
        assert_eq!(tree_map.get(0, 3), Some(Square::Tree));
        assert_eq!(tree_map.get(2, 3), Some(Square::Open));
        assert_eq!(tree_map.get(2, 7), Some(Square::Open));
        assert_eq!(tree_map.get(3, 7), None);
    }

    #[test]
    fn test_trees_hit() {
        let tree_map = indoc! {"
            ..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#
        "}
        .parse::<TreeMap>()
        .unwrap();

        assert_eq!(tree_map.count_trees_hit(3, 1), 7);
    }
}
