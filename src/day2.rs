use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use tracing::warn;

pub fn run() -> io::Result<()> {
    println!("Day 2: Cube Conundrum\n");
    part1()?;
    // 2293 < ANSWER < 2744

    // part2()?;

    Ok(())
}

// Part 1
pub fn part1() -> io::Result<()> {
    let path = Path::new("src/resources/day2.txt");
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let mut total_sum = 0;

    for line in reader.lines() {
        let line = line?;
        // println!("  Line: {}", line);
        if is_game_possible(&line, max_red, max_green, max_blue) {
            // println!("  True\n");
            if let Some(game_id) = line.split(':').next() {
                if let Some(id) = game_id.split_whitespace().last() {
                    if let Ok(val) = id.parse::<i32>() {
                        // println!("  Game {} is possible", val);
                        total_sum += val;
                    } else {
                        warn!("  Error: Could not parse game id");
                    }
                } else {
                    warn!("  Error: Could not split game id");
                }
            } else {
                warn!("  Error: Could not split line");
            }
        } else {
            // println!("  False\n");
        }
    }

    println!("  Part 1:");
    println!("  total_sum of Game IDs: {}\n", total_sum);

    Ok(())
}

fn is_game_possible(game: &str, max_red: i32, max_green: i32, max_blue: i32) -> bool {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for subset in game.split(';') {
        // Find the number of red, green, and blue cubes in this subset
        let (r, g, b) = parse_subset(subset);
        red = red.max(r);
        green = green.max(g);
        blue = blue.max(b);
    }

    // true
    red <= max_red && green <= max_green && blue <= max_blue
}

fn parse_subset(subset: &str) -> (i32, i32, i32) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for color in subset.split(',') {
        let parts: Vec<_> = color.split_whitespace().collect();

        if parts.len() == 2 {
            let value = parts[0].parse::<i32>().unwrap();
            let color = parts[1];

            match color {
                "red" => red += value,
                "green" => green += value,
                "blue" => blue += value,
                _ => (),
            }
        }
    }

    // println!("   red: {}, green: {}, blue: {}", red, green, blue);

    (red, green, blue)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_game_possible_positive_1() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert!(is_game_possible(game, 12, 13, 14));
    }

    #[test]
    fn test_is_game_possible_negative_23() {
        let game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert!(!is_game_possible(game, 12, 13, 14));
    }

    #[test]
    fn test_is_game_possible_negative_4() {
        let game = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        assert!(!is_game_possible(game, 12, 13, 14));
    }

    #[test]
    fn test_parse_subset() {
        let subset = "3 blue, 4 red";
        assert_eq!(parse_subset(subset), (4, 0, 3));
    }

    #[test]
    fn test_run() {
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;
        let mut total_sum = 0;

        let games: Vec<&str> = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        for line in games {
            if is_game_possible(line, max_red, max_green, max_blue) {
                if let Some(game_id) = line.split(':').next() {
                    if let Some(id) = game_id.split_whitespace().last() {
                        if let Ok(val) = id.parse::<i32>() {
                            total_sum += val;
                        } else {
                            warn!("  Error: Could not parse game id");
                        }
                    } else {
                        warn!("  Error: Could not split game id");
                    }
                } else {
                    warn!("  Error: Could not split line");
                }
            }
        }

        assert_eq!(total_sum, 8);
    }
}

//
