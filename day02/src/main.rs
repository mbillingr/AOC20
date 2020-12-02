use common::input::Input;
use common::regex::Regex;

fn main() {
    let input = Input::from_file("data/day02-input.txt");

    let line_parser = LineParser::new();

    let lines: Vec<_> = input.iter_lines()
        .map(|line| line_parser.build(line))
        .collect();

    let n_valid = lines.iter()
        .filter(|pl| pl.is_valid_part1())
        .count();

    println!("Part 1: {}", n_valid);

    let n_valid = lines.iter()
        .filter(|pl| pl.is_valid_part2())
        .count();

    println!("Part 2: {}", n_valid);
}


struct LineParser {
    re: Regex
}

impl LineParser {
    pub fn new() -> Self {
        LineParser {
            re: Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]*)").unwrap()
        }
    }

    pub fn build(&self, line: &str) -> PwdLine {
        let cap = self.re.captures_iter(line).next().unwrap();
        PwdLine {
            first: cap.get(1).unwrap().as_str().parse().unwrap(),
            second: cap.get(2).unwrap().as_str().parse().unwrap(),
            ch: cap.get(3).unwrap().as_str().chars().next().unwrap(),
            pw: cap.get(4).unwrap().as_str().to_string(),
        }
    }
}


#[derive(Debug, Clone)]
struct PwdLine {
    first: usize,
    second: usize,
    ch: char,
    pw: String,
}

impl PwdLine {
    fn is_valid_part1(&self) -> bool {
        let count = self.pw.chars()
            .filter(|ch| ch == &self.ch)
            .count();
        count >= self.first && count <= self.second
    }

    fn is_valid_part2(&self) -> bool {
        let ch1 = self.pw.chars().nth(self.first - 1).unwrap();
        let ch2 = self.pw.chars().nth(self.second - 1).unwrap();
        (ch1 == self.ch) ^ (ch2 == self.ch)
    }
}
