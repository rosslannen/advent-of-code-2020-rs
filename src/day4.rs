use std::collections::HashMap;
use std::convert::{TryFrom, TryInto as _};
use std::fmt;
use std::str::FromStr;

use anyhow::bail;

const BIRTH_YEAR: &'static str = "byr";
const ISSUE_YEAR: &'static str = "iyr";
const EXPIRATION_YEAR: &'static str = "eyr";
const HEIGHT: &'static str = "hgt";
const HAIR_COLOR: &'static str = "hcl";
const EYE_COLOR: &'static str = "ecl";
const ID: &'static str = "pid";
const COUNTRY_ID: &'static str = "cid";

macro_rules! year {
    ($year:ident, $range:pat) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        struct $year(i32);

        impl TryFrom<i32> for $year {
            type Error = anyhow::Error;

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                match value {
                    $range => Ok(Self(value)),
                    _ => Err(anyhow::anyhow!("Invalid birth year: {}", value)),
                }
            }
        }

        impl FromStr for $year {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(s.parse::<i32>()?.try_into()?)
            }
        }
    };
}

year!(BirthYear, 1920..=2002);
year!(IssueYear, 2010..=2020);
year!(ExpirationYear, 2020..=2030);

#[derive(PartialEq, Debug, Clone, Copy)]
enum LengthUnit {
    Centimeters,
    Inches,
}

impl FromStr for LengthUnit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let unit = match s {
            "cm" => Self::Centimeters,
            "in" => Self::Inches,
            _ => bail!("Unrecognized unit: {}", s),
        };

        Ok(unit)
    }
}

impl fmt::Display for LengthUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Centimeters => "cm",
            Self::Inches => "in",
        };
        f.write_str(s)
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Height {
    value: i32,
    unit: LengthUnit,
}

impl FromStr for Height {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.len() - 2;
        let unit = s
            .get(i..)
            .ok_or_else(|| anyhow::anyhow!("Could not parse height unit"))?
            .parse()?;

        let value: i32 = s
            .get(..i)
            .ok_or_else(|| anyhow::anyhow!("Could not parse height value"))?
            .parse()?;

        let range = match unit {
            LengthUnit::Centimeters => (150..=193),
            LengthUnit::Inches => (59..=76),
        };

        if range.contains(&value) {
            Ok(Self { value, unit })
        } else {
            bail!("Value {} outside of range for unit {}", value, unit);
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct HairColor(i32);

impl FromStr for HairColor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('#') {
            bail!("Hair color must start with a '#'");
        }

        let rest = s.get(1..).ok_or_else(|| anyhow::anyhow!("Cannot parse rest of hair color"))?;

        if rest.len() != 6 {
            bail!("Hair color must be 6 digits");
        }

        let value = i32::from_str_radix(rest, 16)?;

        Ok(Self(value))
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum EyeColor {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}

impl FromStr for EyeColor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color = match s {
            "amb" => Self::Amber,
            "blu" => Self::Blue,
            "brn" => Self::Brown,
            "gry" => Self::Grey,
            "grn" => Self::Green,
            "hzl" => Self::Hazel,
            "oth" => Self::Other,
            _ => bail!("Unrecognized eye color: {}", s),
        };

        Ok(color)
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct PassportId(i32);

impl FromStr for PassportId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 9 {
            bail!("Passport id must be 9 digits");
        }

        Ok(Self(s.parse()?))
    }
}

#[derive(PartialEq, Debug)]
struct Passport {
    birth_year: BirthYear,
    issue_year: IssueYear,
    expiration_year: ExpirationYear,
    height: Height,
    hair_color: HairColor,
    eye_color: EyeColor,
    id: PassportId,
    country_id: Option<String>,
}

impl FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: HashMap<_, _> = s
            .split_whitespace()
            .filter_map(|p| {
                let mut pair_iter = p.split(':');

                Some((pair_iter.next()?, pair_iter.next()?))
            })
            .collect();

        let get_str = |key| -> anyhow::Result<&str> {
            let value: &&str = items
                .get(key)
                .ok_or_else(|| anyhow::anyhow!("No value found for {}", key))?;

            Ok(*value)
        };

        let passport = Passport {
            birth_year: get_str(BIRTH_YEAR)?.parse()?,
            issue_year: get_str(ISSUE_YEAR)?.parse()?,
            expiration_year: get_str(EXPIRATION_YEAR)?.parse()?,
            height: get_str(HEIGHT)?.parse()?,
            hair_color: get_str(HAIR_COLOR)?.parse()?,
            eye_color: get_str(EYE_COLOR)?.parse()?,
            id: get_str(ID)?.parse()?,
            country_id: items.get(COUNTRY_ID).map(|s| String::from(*s)),
        };

        Ok(passport)
    }
}

#[derive(PartialEq, Debug)]
struct SimplePassport {
    birth_year: String,
    issue_year: String,
    expiration_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
    id: String,
    country_id: Option<String>,
}

impl FromStr for SimplePassport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: HashMap<_, _> = s
            .split_whitespace()
            .filter_map(|p| {
                let mut pair_iter = p.split(':');

                Some((pair_iter.next()?, pair_iter.next()?))
            })
            .collect();

        let get_str = |key| -> anyhow::Result<&str> {
            let value: &&str = items
                .get(key)
                .ok_or_else(|| anyhow::anyhow!("No value found for {}", key))?;

            Ok(*value)
        };

        let passport = Self {
            birth_year: get_str(BIRTH_YEAR)?.to_string(),
            issue_year: get_str(ISSUE_YEAR)?.to_string(),
            expiration_year: get_str(EXPIRATION_YEAR)?.to_string(),
            height: get_str(HEIGHT)?.to_string(),
            hair_color: get_str(HAIR_COLOR)?.to_string(),
            eye_color: get_str(EYE_COLOR)?.to_string(),
            id: get_str(ID)?.to_string(),
            country_id: items.get(COUNTRY_ID).map(|s| String::from(*s)),
        };

        Ok(passport)
    }
}

pub fn part1(raw_input: &str) -> anyhow::Result<usize> {
    Ok(raw_input
        .split("\n\n")
        .filter_map(|sequence| sequence.parse::<SimplePassport>().ok())
        .count())
}

pub fn part2(raw_input: &str) -> anyhow::Result<usize> {
    Ok(raw_input
        .split("\n\n")
        .filter_map(|sequence| sequence.parse::<Passport>().ok())
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_parse_valid_passport() {
        let input = indoc! {"
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm
        "};

        assert_eq!(
            input.parse::<Passport>().unwrap(),
            Passport {
                birth_year: BirthYear(1937),
                issue_year: IssueYear(2017),
                expiration_year: ExpirationYear(2020),
                height: Height {
                    value: 183,
                    unit: LengthUnit::Centimeters,
                },
                hair_color: HairColor(0xfffffd),
                eye_color: EyeColor::Grey,
                id: PassportId(860033327),
                country_id: Some(String::from("147")),
            }
        );
    }

    #[test]
    fn test_parse_birth_year() {
        assert_eq!(BirthYear::from_str("2000").unwrap(), BirthYear(2000));
        assert!(BirthYear::from_str("5").is_err());
    }
}
