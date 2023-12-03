use std::{
    cmp,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
struct Set {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

impl Set {
    fn new(line: &str) -> Set {
        let mut set = Set {
            red: None,
            green: None,
            blue: None,
        };

        line.split(",")
            .map(|c| c.trim())
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|c| {
                let cube = c.split(" ").collect::<Vec<&str>>();
                let val = cube[0].parse::<u32>().unwrap_or_default();
                match cube[1] {
                    "red" => set.red = Some(val),
                    "green" => set.green = Some(val),
                    "blue" => set.blue = Some(val),
                    _ => (),
                }
            });

        set
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn new(line: String) -> Game {
        let game_and_sets: Vec<&str> = line.split(":").collect();
        Game {
            id: game_and_sets[0]
                .split(" ")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            sets: game_and_sets[1]
                .split(";")
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| Set::new(s.trim()))
                .collect::<Vec<Set>>(),
        }
    }
}

#[derive(Debug)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn is_game_possible(&self, game: &Game) -> bool {
        for set in &game.sets {
            if set.red.unwrap_or_default() > self.red
                || set.green.unwrap_or_default() > self.green
                || set.blue.unwrap_or_default() > self.blue
            {
                return false;
            }
        }

        true
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    fn find_smallest(game: &Game) -> Bag {
        let (red, green, blue) = game.sets.iter().fold((0, 0, 0), |(r, g, b), s| {
            (
                cmp::max(r, s.red.unwrap_or_default()),
                cmp::max(g, s.green.unwrap_or_default()),
                cmp::max(b, s.blue.unwrap_or_default()),
            )
        });

        Bag { red, green, blue }
    }
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

fn main() -> io::Result<()> {
    println!("day2");

    let path = Path::new("./bin/day2/input");
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let games = lines
        .into_iter()
        .map(|l| Game::new(l.unwrap()))
        .collect::<Vec<Game>>();

    println!("games {}", games.len());

    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    let ids_sum = games
        .iter()
        .filter(|g| bag.is_game_possible(g))
        .fold(0, |acc, g| {
            println!("{}", g.id);
            acc + g.id
        });

    println!("sum of ids {}", ids_sum);

    let smallest_bag_power = games
        .iter()
        .map(|g| Bag::find_smallest(g))
        .map(|b| b.power())
        .fold(0, |acc, b| acc + b);

    println!("smallest bag power sum {}", smallest_bag_power);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    //
    // Bag contins 12 red, 13 green, 14 blue
    // Only game 1, 2, 5 are possible

    #[test]
    fn test_load_game() {
        let game = Game::new(
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
        );
        assert_eq!(game.id, 2);
        assert_eq!(game.sets.len(), 3);
        assert_eq!(game.sets[0].red, None);
        assert_eq!(game.sets[0].green, Some(2));
        assert_eq!(game.sets[0].blue, Some(1));
        assert_eq!(game.sets[1].red, Some(1));
        assert_eq!(game.sets[1].green, Some(3));
        assert_eq!(game.sets[1].blue, Some(4));
        assert_eq!(game.sets[2].red, None);
        assert_eq!(game.sets[2].green, Some(1));
        assert_eq!(game.sets[2].blue, Some(1));
    }

    #[test]
    fn test_find() {
        let games = vec![
            Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()),
            Game::new(
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            ),
            Game::new(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                    .to_string(),
            ),
            Game::new(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                    .to_string(),
            ),
            Game::new("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()),
            Game::new("Game 5: 6 red, 14 blue, 3 green; 2 blue, 6 red, 12 green".to_string()),
        ];

        let bag = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };

        assert_eq!(bag.is_game_possible(&games[0]), true);
        assert_eq!(bag.is_game_possible(&games[1]), true);
        assert_eq!(bag.is_game_possible(&games[2]), false);
        assert_eq!(bag.is_game_possible(&games[3]), false);
        assert_eq!(bag.is_game_possible(&games[4]), true);
        assert_eq!(bag.is_game_possible(&games[5]), true);
    }

    #[test]
    fn test_fewest_cubes_needed() {
        let games = vec![
            Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()),
            Game::new(
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            ),
            Game::new(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                    .to_string(),
            ),
            Game::new(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                    .to_string(),
            ),
            Game::new("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()),
            Game::new("Game 5: 6 red, 14 blue, 3 green; 2 blue, 6 red, 12 green".to_string()),
        ];

        assert_eq!(
            Bag::find_smallest(&games[0]),
            Bag {
                red: 4,
                green: 2,
                blue: 6
            }
        );
        assert_eq!(
            Bag::find_smallest(&games[1]),
            Bag {
                red: 1,
                green: 3,
                blue: 4
            }
        );
        assert_eq!(
            Bag::find_smallest(&games[2]),
            Bag {
                red: 20,
                green: 13,
                blue: 6
            }
        );
        assert_eq!(
            Bag::find_smallest(&games[3]),
            Bag {
                red: 14,
                green: 3,
                blue: 15
            }
        );
        assert_eq!(
            Bag::find_smallest(&games[4]),
            Bag {
                red: 6,
                green: 3,
                blue: 2
            }
        );

        assert_eq!(Bag::find_smallest(&games[0]).power(), 48);
        assert_eq!(Bag::find_smallest(&games[1]).power(), 12);
        assert_eq!(Bag::find_smallest(&games[2]).power(), 1560);
        assert_eq!(Bag::find_smallest(&games[3]).power(), 630);
        assert_eq!(Bag::find_smallest(&games[4]).power(), 36);
    }
}
