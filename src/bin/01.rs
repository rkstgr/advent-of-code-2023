use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(1);

fn extract_calibration_value(line: &str) -> Option<u32> {
    let first_digit = line.chars().find(|c| c.is_digit(10))?.to_digit(10)?;
    let last_digit = line.chars().rev().find(|c| c.is_digit(10))?.to_digit(10)?;

    Some(first_digit * 10 + last_digit)
}

pub fn part_one(input: &str) -> Option<u32> {
    input.lines().map(extract_calibration_value).sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"[1-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let number_map: HashMap<&str, char> = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]
    .iter()
    .cloned()
    .collect();

    let calibration_numbers = input.lines().filter_map(|line| {
        let first_number = re.captures_iter(line).next().and_then(|cap| {
            cap.get(0).and_then(|mat| {
                if mat.as_str().chars().all(|c| c.is_digit(10)) {
                    mat.as_str().parse::<u32>().ok()
                } else {
                    number_map
                        .get(mat.as_str())
                        .copied()
                        .map(|c| c.to_digit(10).unwrap())
                }
            })
        });

        let re2 = Regex::new(r"[1-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
        let second_number = re2
            .captures_iter(&line.chars().rev().collect::<String>())
            .next()
            .and_then(|cap| {
                cap.get(0).and_then(|mat| {
                    if mat.as_str().chars().all(|c| c.is_digit(10)) {
                        mat.as_str().parse::<u32>().ok()
                    } else {
                        number_map
                            .get(mat.as_str().chars().rev().collect::<String>().as_str())
                            .copied()
                            .map(|c| c.to_digit(10).unwrap())
                    }
                })
            })
            .unwrap();

        Some(first_number.unwrap() * 10 + second_number)
    });

    Some(calibration_numbers.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
