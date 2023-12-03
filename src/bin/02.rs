use std::collections::HashMap;
advent_of_code::solution!(2);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

pub fn part_one(input: &str) -> Option<u32> {
    let max_cubes = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);

    let mut sum_of_ids = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let game_id: u32 = parts[0].replace("Game ", "").parse().unwrap();
        let rounds = parts[1].split(';');

        let mut possible = true;
        for round in rounds {
            let cubes: HashMap<Color, u32> = round
                .split(',')
                .map(|c| c.trim())
                .map(|c| {
                    let parts: Vec<&str> = c.split_whitespace().collect();
                    let number = parts[0].parse::<u32>().unwrap();
                    let color = match parts[1] {
                        "red" => Color::Red,
                        "green" => Color::Green,
                        "blue" => Color::Blue,
                        _ => panic!("Unknown color"),
                    };
                    (color, number)
                })
                .collect();

            for (&color, &number) in cubes.iter() {
                if number > *max_cubes.get(&color).unwrap() {
                    possible = false;
                    break;
                }
            }

            if !possible {
                break;
            }
        }

        if possible {
            sum_of_ids += game_id;
        }
    }

    Some(sum_of_ids)
}

fn parse_cubes_info(info: &str) -> HashMap<Color, u32> {
    info.split(',')
        .map(|c| c.trim())
        .map(|c| {
            let parts: Vec<&str> = c.split_whitespace().collect();
            let number = parts[0].parse::<u32>().unwrap();
            let color = match parts[1] {
                "red" => Color::Red,
                "green" => Color::Green,
                "blue" => Color::Blue,
                _ => panic!("Unknown color"),
            };
            (color, number)
        })
        .collect()
}

fn calculate_power(min_cubes: &HashMap<Color, u32>) -> u32 {
    min_cubes.get(&Color::Red).unwrap_or(&0)
        * min_cubes.get(&Color::Green).unwrap_or(&0)
        * min_cubes.get(&Color::Blue).unwrap_or(&0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut powers_sum = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let rounds = parts[1].split(';');

        // Initialize the minimum cubes with zero
        let mut min_cubes: HashMap<Color, u32> = HashMap::new();

        for round in rounds {
            let cubes = parse_cubes_info(round);

            for (&color, &number) in cubes.iter() {
                let entry = min_cubes.entry(color).or_insert(0);
                if number > *entry {
                    *entry = number;
                }
            }
        }

        powers_sum += calculate_power(&min_cubes);
    }

    Some(powers_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
