use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

struct Grid {
    rows: Vec<Vec<char>>,
}

impl Grid {
    fn new() -> Grid {
        Grid { rows: Vec::new() }
    }

    fn add_row(&mut self, row: &String) {
        self.rows.push(row.chars().collect::<Vec<char>>());
    }

    fn verify(&self) -> bool {
        let first_len = &self.rows[0].len();
        for row in &self.rows {
            if &row.len() != first_len {
                return false;
            }
        }

        true
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.rows[y][x]
    }

    fn get_protected(&self, x: usize, y: usize, add_x: i32, add_y: i32) -> char {
        let new_x = x as i32 + add_x;
        let new_y = y as i32 + add_y;

        if new_x < 0
            || new_y < 0
            || new_x as usize >= self.rows[y].len()
            || new_y as usize >= self.rows.len()
        {
            return '.';
        }

        self.rows[(y as i32 + add_y) as usize][(x as i32 + add_x) as usize]
    }

    fn is_symbol_adjacent(&self, x: usize, y: usize, len: usize) -> bool {
        let mut symbol_found = false;

        for i in 0..len + 2 {
            if self.get_protected(x, y, i as i32 - 1, -1) != '.'
                || self.get_protected(x, y, i as i32 - 1, 1) != '.'
            {
                symbol_found = true;
            }
        }

        if self.get_protected(x, y, -1, 0) != '.' || self.get_protected(x, y, len as i32, 0) != '.'
        {
            symbol_found = true;
        }

        symbol_found
    }

    fn find_next_num_in_row(&self, x: usize, y: usize) -> Option<(u32, usize, usize, usize)> {
        let mut num: Vec<char> = Vec::new();
        let mut pos: Option<(usize, usize)> = None;

        if y >= self.rows.len() {
            return None;
        }

        for x in x..self.rows[y].len() {
            let n = self.get(x, y);
            if num.len() > 0 && !n.is_numeric() {
                break;
            }

            if n.is_numeric() {
                num.push(n);
                if pos == None {
                    pos = Some((x, y));
                }
            }
        }

        if num.len() > 0 {
            Some((
                (&num)
                    .into_iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap(),
                pos.unwrap().0,
                pos.unwrap().1,
                num.len(),
            ))
        } else {
            None
        }
    }

    fn find_next_num(&self, x: usize, y: usize) -> Option<(u32, usize, usize, usize)> {
        if let Some(val) = self.find_next_num_in_row(x, y) {
            return Some(val);
        }

        for y in y + 1..self.rows.len() {
            if let Some(val) = self.find_next_num_in_row(0, y) {
                return Some(val);
            }
        }

        None
    }

    fn find_all_adjacent_numbers(&self) -> Vec<u32> {
        let mut next_x: usize = 0;
        let mut next_y: usize = 0;
        let mut nums: Vec<u32> = Vec::new();
        while let Some((num, x, y, len)) = self.find_next_num(next_x, next_y) {
            if self.is_symbol_adjacent(x, y, len) {
                nums.push(num);
            }

            next_x = x + len;
            next_y = y;
        }

        nums
    }
}

fn main() -> io::Result<()> {
    println!("day3");

    let path = Path::new("./bin/day3/input");
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let mut grid = Grid::new();
    for line in lines {
        grid.add_row(&line?);
    }
    grid.verify();

    let nums = grid.find_all_adjacent_numbers();
    let sum = nums.iter().fold(0, |acc, n| acc + n);
    println!("sum {}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // 467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..

    // 114 not adjacent to symbol
    // 58 not adjacent to symbol

    // Total sum is 4361

    #[test]
    fn test_load() {
        let mut grid = Grid::new();
        grid.add_row(&"467..114..".to_string());
        grid.add_row(&"...*......".to_string());
        grid.add_row(&"..35..633.".to_string());
        grid.add_row(&"......#...".to_string());
        grid.add_row(&"617*......".to_string());
        grid.add_row(&".....+.58.".to_string());
        grid.add_row(&"..592.....".to_string());
        grid.add_row(&"......755.".to_string());
        grid.add_row(&"...$.*....".to_string());
        grid.add_row(&".664.598..".to_string());
        assert!(grid.verify());
    }

    #[test]
    fn test_symbol_not_adjacent() {
        let mut grid = Grid::new();
        grid.add_row(&".....".to_string());
        grid.add_row(&".123.".to_string());
        grid.add_row(&".....".to_string());
        grid.verify();
        assert_eq!(grid.is_symbol_adjacent(1, 1, 3), false);
    }

    #[test]
    fn test_symbol_adjacent_simple() {
        let mut grid = Grid::new();
        grid.add_row(&"...".to_string());
        grid.add_row(&".1.".to_string());
        grid.add_row(&"..$".to_string());
        grid.verify();
        assert!(grid.is_symbol_adjacent(1, 1, 1));
    }

    #[test]
    fn test_symbol_adjacent_simple_not_found() {
        let mut grid = Grid::new();
        grid.add_row(&"...".to_string());
        grid.add_row(&".1.".to_string());
        grid.add_row(&"...".to_string());
        grid.verify();
        assert_eq!(grid.is_symbol_adjacent(1, 1, 1), false);
    }

    #[test]
    fn test_symbol_adjacent_simple_step2() {
        let mut grid = Grid::new();
        grid.add_row(&"....".to_string());
        grid.add_row(&".12.".to_string());
        grid.add_row(&"..$.".to_string());
        grid.verify();
        assert!(grid.is_symbol_adjacent(1, 1, 2));
    }

    #[test]
    fn test_symbol_adjacent_corner() {
        let mut grid = Grid::new();
        grid.add_row(&".....".to_string());
        grid.add_row(&".123.".to_string());
        grid.add_row(&"....$".to_string());
        grid.verify();
        assert!(grid.is_symbol_adjacent(1, 1, 3));
    }

    #[test]
    fn test_symbol_adjacent_top() {
        let mut grid = Grid::new();
        grid.add_row(&"..*..".to_string());
        grid.add_row(&".123.".to_string());
        grid.add_row(&".....".to_string());
        grid.verify();
        assert!(grid.is_symbol_adjacent(1, 1, 3));
    }

    #[test]
    fn test_symbol_adjacent_left() {
        let mut grid = Grid::new();
        grid.add_row(&".....".to_string());
        grid.add_row(&"$123.".to_string());
        grid.add_row(&".....".to_string());
        grid.verify();
        assert!(grid.is_symbol_adjacent(1, 1, 3));
    }

    #[test]
    fn test_symbol_adjacent_right() {
        let mut grid = Grid::new();
        grid.add_row(&".....".to_string());
        grid.add_row(&".123*".to_string());
        grid.add_row(&".....".to_string());
        grid.verify();
        assert!(grid.is_symbol_adjacent(1, 1, 3));
    }

    #[test]
    fn test_find_next_num_single_row() {
        let mut grid = Grid::new();
        grid.add_row(&"467..11..".to_string());
        assert_eq!(grid.find_next_num(0, 0), Some((467, 0, 0, 3)));
        assert_eq!(grid.find_next_num(3, 0), Some((11, 5, 0, 2)));
    }

    #[test]
    fn test_find_next_num_multi_row() {
        let mut grid = Grid::new();
        grid.add_row(&"467..114..".to_string());
        grid.add_row(&"...*......".to_string());
        grid.add_row(&"..35..633.".to_string());
        grid.add_row(&"......#...".to_string());
        assert_eq!(grid.find_next_num(0, 0), Some((467, 0, 0, 3)));
        assert_eq!(grid.find_next_num(3, 0), Some((114, 5, 0, 3)));
        assert_eq!(grid.find_next_num(8, 0), Some((35, 2, 2, 2)));
        assert_eq!(grid.find_next_num(4, 2), Some((633, 6, 2, 3)));
        assert_eq!(grid.find_next_num(9, 2), None);
    }

    #[test]
    fn test_find_next_num_multi_row_right() {
        let mut grid = Grid::new();
        grid.add_row(&"467....114".to_string());
        grid.add_row(&"...23.....".to_string());
        assert_eq!(grid.find_next_num(0, 0), Some((467, 0, 0, 3)));
        assert_eq!(grid.find_next_num(3, 0), Some((114, 7, 0, 3)));
        assert_eq!(grid.find_next_num(10, 0), Some((23, 3, 1, 2)));
        assert_eq!(grid.find_next_num(9, 2), None);
    }

    #[test]
    fn test_adjacent_3_true_1_false() {
        let mut grid = Grid::new();
        grid.add_row(&"467..114..".to_string());
        grid.add_row(&"...*......".to_string());
        grid.add_row(&"..35..633.".to_string());
        grid.add_row(&"......#...".to_string());
        assert_eq!(grid.is_symbol_adjacent(0, 0, 3), true);
        assert_eq!(grid.is_symbol_adjacent(5, 0, 3), false);
        assert_eq!(grid.is_symbol_adjacent(2, 2, 2), true);
        assert_eq!(grid.is_symbol_adjacent(6, 2, 3), true);
    }

    #[test]
    fn test_find_all_adjacent_numbers() {
        let mut grid = Grid::new();
        grid.add_row(&"467..114..".to_string());
        grid.add_row(&"...*......".to_string());
        grid.add_row(&"..35..633.".to_string());
        grid.add_row(&"......#...".to_string());
        grid.add_row(&"617*......".to_string());
        grid.add_row(&".....+.58.".to_string());
        grid.add_row(&"..592.....".to_string());
        grid.add_row(&"......755.".to_string());
        grid.add_row(&"...$.*....".to_string());
        grid.add_row(&".664.598..".to_string());
        assert!(grid.verify());

        let nums = grid.find_all_adjacent_numbers();
        assert_eq!(nums[0], 467);
        assert_eq!(nums[1], 35);
        assert_eq!(nums[2], 633);
        assert_eq!(nums[3], 617);
        assert_eq!(nums[4], 592);
        assert_eq!(nums[5], 755);
        assert_eq!(nums[6], 664);
        assert_eq!(nums[7], 598);

        let sum = nums.iter().fold(0, |acc, n| acc + n);
        assert_eq!(sum, 4361);
    }
}
