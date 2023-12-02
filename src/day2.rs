use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() -> io::Result<()> {
    println!("Day 2: Cube Conundrum\n");
    part1()?;

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

    let games_str: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let game_map = build_game_map(games_str);

    for (game_id, subsets) in game_map {
        if is_game_possible(subsets, max_red, max_green, max_blue) {
            total_sum += game_id.parse::<i32>().unwrap();
        }
    }

    println!("  Part 1:");
    println!("  total_sum of Game IDs: {}\n", total_sum);

    Ok(())
}

fn build_game_map(games: Vec<String>) -> HashMap<String, Vec<(i32, i32, i32)>> {
    let mut game_map = HashMap::new();

    for game in games {
        // Extract the game id
        let game_id = game.split(':').next().unwrap().to_string();
        let game_id = game_id.split_whitespace().last().unwrap().to_string();
        let game_id = game_id.parse::<i32>().unwrap();

        // Gather the subsets for this game
        let mut subsets = Vec::new();
        for subset in game.split(':').last().unwrap().split(';') {
            subsets.push(parse_subset(subset.trim()));
        }

        // Insert the subsets into the game map
        game_map.insert(game_id.to_string(), subsets);
    }

    game_map
}

fn is_game_possible(
    game: Vec<(i32, i32, i32)>,
    max_red: i32,
    max_green: i32,
    max_blue: i32,
) -> bool {
    // Init the max game values
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    // Find the max values for across game subsets
    for subset in game {
        red = red.max(subset.0);
        green = green.max(subset.1);
        blue = blue.max(subset.2);
    }

    red <= max_red && green <= max_green && blue <= max_blue
}

fn parse_subset(subset: &str) -> (i32, i32, i32) {
    // Init the subset values
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    // Parse the subset
    for color in subset.split(',') {
        let parts: Vec<_> = color.split_whitespace().collect();

        // If the subset is valid, parse the value and color
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

    (red, green, blue)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_game_possible_positive_1() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let game = build_game_map(vec![game.to_string()]);
        let game = game.get("1").unwrap();

        assert!(is_game_possible(game.to_owned(), 12, 13, 14));
    }

    #[test]
    fn test_is_game_possible_negative_23() {
        let game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";

        let game = build_game_map(vec![game.to_string()]);
        let game = game.get("3").unwrap();

        assert!(!is_game_possible(game.to_owned(), 12, 13, 14));
    }

    #[test]
    fn test_is_game_possible_negative_4() {
        let game = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";

        let game = build_game_map(vec![game.to_string()]);
        let game = game.get("4").unwrap();

        assert!(!is_game_possible(game.to_owned(), 12, 13, 14));
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

        let games: Vec<String> = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_owned(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_owned(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_owned(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_owned(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_owned(),
        ];

        let games = build_game_map(games.to_vec());

        for (game_id, subsets) in games {
            if is_game_possible(subsets, max_red, max_green, max_blue) {
                total_sum += game_id.parse::<i32>().unwrap();
            }
        }
        assert_eq!(total_sum, 8);
    }
}

//
