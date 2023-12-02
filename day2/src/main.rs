use regex::Regex;
use std::{cmp::max, collections::HashMap, env, fs};

fn get_game_id(game_line: &str) -> u32 {
    let game_re = Regex::new(r"Game (\d+):").unwrap();
    game_re
        .captures(game_line)
        .expect("every line should start with a game id")
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .expect("the game id should be numeric")
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
enum Color {
    Green,
    Blue,
    Red,
}

static ALL_COLORS: [Color; 3] = [Color::Green, Color::Blue, Color::Red];

impl Color {
    fn as_str(&self) -> &'static str {
        match self {
            Color::Green => "green",
            Color::Blue => "blue",
            Color::Red => "red",
        }
    }
}
type CubeSet = HashMap<Color, u32>;

fn get_color_count(set: &CubeSet, color: Color) -> u32 {
    *set.get(&color).unwrap_or(&0)
}

fn set_is_ok(set: &CubeSet) -> bool {
    get_color_count(set, Color::Red) <= 12
        && get_color_count(set, Color::Green) <= 13
        && get_color_count(set, Color::Blue) <= 14
}

fn extract_color(chunk: &str, color: Color, set: &mut CubeSet) -> Option<()> {
    let re = Regex::new(format!(r"(\d+) {}", color.as_str()).as_str()).unwrap();
    if let Ok(num) = re.captures(chunk)?.get(1)?.as_str().parse::<u32>() {
        *set.entry(color).or_insert(0) += num;
    }
    Some(())
}

fn create_set(set_string: &str) -> CubeSet {
    let mut set = CubeSet::new();
    set_string.split(",").for_each(|chunk: &str| {
        ALL_COLORS.iter().for_each(|color| {
            extract_color(chunk, *color, &mut set);
        });
    });
    set
}

fn part_one(game_line: &str) -> u32 {
    if game_line.is_empty() {
        return 0;
    }
    let game_id = get_game_id(game_line);
    if game_line
        .split(";")
        .all(|set_string: &str| set_is_ok(&create_set(set_string)))
    {
        return game_id;
    }
    0
}

fn part_two(game_line: &str) -> u32 {
    if game_line.is_empty() {
        return 0;
    }
    let max_set = game_line
        .split(";")
        .map(create_set)
        .reduce(|acc, cur| {
            let mut new_set = CubeSet::new();
            ALL_COLORS.iter().for_each(|color| {
                new_set.insert(
                    *color,
                    max(get_color_count(&acc, *color), get_color_count(&cur, *color)),
                );
            });

            new_set
        })
        .unwrap();

    get_color_count(&max_set, Color::Green)
        * get_color_count(&max_set, Color::Blue)
        * get_color_count(&max_set, Color::Red)
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let content = fs::read_to_string(file_path).unwrap();

    let r: u32 = content.split("\n").map(part_two).sum();
    // let r: u32 = content.split("\n").map(part_one).sum();

    println!("the answer is: {r}");
}
