use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn find_first_number(line: &String, reverse: bool) -> Option<(usize, u32)> {
    let index_str = match reverse {
        true => line.rfind(char::is_numeric),
        false => line.find(char::is_numeric),
    };

    if let Some(i) = index_str {
        let mut index = i;
        if reverse {
            index = line.len() - i - 1;
        }

        return Some((index, line.chars().nth(i).unwrap().to_digit(10).unwrap()));
    }

    None
}

fn find_first_word_number(line: &String, reverse: bool) -> Option<(usize, u32)> {
    let words: Vec<String> = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .map(|s| s.to_string())
    .map(|s| {
        if reverse {
            s.chars().rev().collect()
        } else {
            s
        }
    })
    .collect();

    let processed_line = if reverse {
        line.chars().rev().collect::<String>()
    } else {
        line.to_string()
    };

    let mut found_numbers = words
        .iter()
        .map(|s| (s.clone(), processed_line.find(s)))
        .filter(|(_, i)| i.is_some())
        .collect::<Vec<(String, Option<usize>)>>();

    found_numbers.sort_by(|(_, i0), (_, i1)| i0.unwrap().cmp(&i1.unwrap()));

    if found_numbers.len() > 0 {
        let (word, index) = &found_numbers[0];
        
        let mut real_index = index.unwrap();
        if reverse {
            real_index = index.unwrap() + word.len() - 1;
        }

        return Some((
            real_index,
            words.iter().position(|n| n == word).unwrap() as u32 + 1,
        ));
    }

    None
}

fn find_number_by_word_and_digit(line: &String, reverse: bool) -> Option<u32> {
    match (
        find_first_number(&line, reverse),
        find_first_word_number(&line, reverse),
    ) {
        (None, Some((i, n))) => {
            Some(n)
        },
        (Some((i, n)), None) => {
            Some(n)
        },
        (Some((i0, n0)), Some((i1, n1))) => {
            if i0 < i1 {
                Some(n0)
            } else {
                Some(n1)
            }
        }
        (None, None) => None,
    }
}

fn main() -> io::Result<()> {
    println!("day1");

    let path = Path::new("./bin/day1/input");
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let mut acc: u32 = 0;
    for l in lines {
        let line = l?;

        let mut num = "".to_string();
        
        let first_number = find_number_by_word_and_digit(&line, false);
        if let Some(n) = first_number {
            num.push(n.to_string().chars().nth(0).unwrap());
        }

        let last_number = find_number_by_word_and_digit(&line, true);
        if let Some(n) = last_number {
            num.push(n.to_string().chars().nth(0).unwrap());
        }

        println!(
            "{} {} {} {}",
            line,
            num,
            first_number.unwrap(),
            last_number.unwrap()
        );
        acc += num.parse::<u32>().unwrap();
    }

    println!("acc {}", acc);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{find_first_number, find_first_word_number, find_number_by_word_and_digit};

    #[test]
    fn test_find_first_number() {
        assert_eq!(find_first_number(&"1abc2".to_string(), false), Some((0, 1)));
        assert_eq!(
            find_first_number(&"pqr3stu8vwx".to_string(), false),
            Some((3, 3))
        );
        assert_eq!(
            find_first_number(&"a1b2c3d4e5f".to_string(), false),
            Some((1, 1))
        );
        assert_eq!(find_first_number(&"treb7uchet".to_string(), false), Some((4, 7)));
    }

    #[test]
    fn test_find_first_number_reverse() {
        assert_eq!(find_first_number(&"1abc2".to_string(), true), Some((0, 2)));
        assert_eq!(find_first_number(&"pqr3stu8vwx".to_string(), true), Some((3, 8)));
        assert_eq!(find_first_number(&"a1b2c3d4e5f".to_string(), true), Some((1, 5)));
        assert_eq!(find_first_number(&"treb7uchet".to_string(), true), Some((5, 7)));
    }

    #[test]
    fn test_find_first_number_fails_when_number() {
        assert_eq!(find_first_number(&"abc".to_string(), false), None);
        assert_eq!(find_first_number(&"trebuchet".to_string(), false), None);
    }

    #[test]
    fn test_find_first_word_number() {
        assert_eq!(
            find_first_word_number(&"eightwothree".to_string(), false),
            Some((0, 8))
        );
        assert_eq!(
            find_first_word_number(&"two1nine".to_string(), false),
            Some((0, 2))
        );
        assert_eq!(
            find_first_word_number(&"abcone2threexyz".to_string(), false),
            Some((3, 1))
        );
        assert_eq!(
            find_first_word_number(&"xtwone3four".to_string(), false),
            Some((1, 2))
        );
        assert_eq!(
            find_first_word_number(&"4nineeightseven2".to_string(), false),
            Some((1, 9))
        );
        assert_eq!(
            find_first_word_number(&"zoneight234".to_string(), false),
            Some((1, 1))
        );
        assert_eq!(
            find_first_word_number(&"7pqrstsixteen".to_string(), false),
            Some((6, 6))
        );
    }

    #[test]
    fn test_find_first_word_number_reverse() {
        assert_eq!(
            find_first_word_number(&"two1nine".to_string(), true),
            Some((3, 9))
        );
        assert_eq!(
            find_first_word_number(&"eightwothree".to_string(), true),
            Some((4, 3))
        );
        assert_eq!(
            find_first_word_number(&"abcone2threexyz".to_string(), true),
            Some((7, 3))
        );
        assert_eq!(
            find_first_word_number(&"xtwone3four".to_string(), true),
            Some((3, 4))
        );
        assert_eq!(
            find_first_word_number(&"4nineeightseven2".to_string(), true),
            Some((5, 7))
        );
        assert_eq!(
            find_first_word_number(&"zoneight234".to_string(), true),
            Some((7, 8))
        );
        assert_eq!(
            find_first_word_number(&"7pqrstsixteen".to_string(), true),
            Some((6, 6))
        );
    }

    #[test]
    fn test_find_first_word_number_none_found() {
        assert_eq!(
            find_first_word_number(&"4nigeeixhhtsevan2".to_string(), true),
            None
        );
    }

    #[test]
    fn test_find_first_word_number_reverse_none_found() {
        assert_eq!(
            find_first_word_number(&"4nigeeixhhtsevan2".to_string(), false),
            None
        );
    }

    #[test]
    fn test_find_first_word_number_and_digit() {
        assert_eq!(
            find_number_by_word_and_digit(&"5bszzkpcdxqkvkf7tgcone2".to_string(), false),
            Some(5)
        );
        assert_eq!(
            find_number_by_word_and_digit(&"4threelfvzndfive".to_string(), false),
            Some(4)
        );
        assert_eq!(
            find_number_by_word_and_digit(
                &"mhrckkcgqdms1rvrfcvpsn3trmfltvbhr4sixlpslr".to_string(),
                false
            ),
            Some(1)
        );
    }

    #[test]
    fn test_find_first_word_number_and_digit_reverse() {
        assert_eq!(
            find_number_by_word_and_digit(&"5bszzkpcdxqkvkf7tgcone2".to_string(), true),
            Some(2)
        );
        assert_eq!(
            find_number_by_word_and_digit(&"4threelfvzndfive".to_string(), true),
            Some(5)
        );
        assert_eq!(
            find_number_by_word_and_digit(
                &"mhrckkcgqdms1rvrfcvpsn3trmfltvbhr4sixlpslr".to_string(),
                true 
            ),
            Some(6)
        );
    }
}
