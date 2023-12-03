use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};


fn main() -> io::Result<()> {
    println!("day2");

    let path = Path::new("./bin/day2/input");
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() {
    }
}
