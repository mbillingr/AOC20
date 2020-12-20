use common::input::Input;
use std::collections::HashMap;

fn main() {
    let input = Input::from_file("data/day19-input.txt");
    let mut input = input.iter_blocks();

    let rules_input = input.next().unwrap();
    let messages_input = input.next().unwrap();

    let mut rules: HashMap<_, _> = rules_input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let rule_nr: u8 = parts.next().unwrap().parse().unwrap();
            let rule = Rule::from_str(parts.next().unwrap());
            (rule_nr, rule)
        })
        .collect();

    {
        let rule0 = &rules[&0];

        let n_matching = messages_input
            .lines()
            .map(str::as_bytes)
            .filter(|msg| rule0.does_match_str_completely(msg, &rules))
            .count();

        println!("Part 1: {}", n_matching);
    }

    {
        rules.remove(&8);
        rules.remove(&11);
        rules.insert(0, Rule::special(42, 31));

        let rule0 = &rules[&0];

        let n_matching = messages_input
            .lines()
            .map(str::as_bytes)
            .filter(|msg| rule0.does_match_str_completely(msg, &rules))
            .count();

        println!("Part 2: {}", n_matching);
    }
}

#[derive(Debug)]
enum Rule {
    Char(u8),
    SubRule(u8),
    Sequence(Box<Rule>, Box<Rule>),
    Alternative(Box<Rule>, Box<Rule>),
    Special(Box<Rule>, Box<Rule>),
}

impl From<u8> for Rule {
    fn from(r: u8) -> Self {
        Rule::SubRule(r)
    }
}

impl Rule {
    fn seq(a: impl Into<Rule>, b: impl Into<Rule>) -> Self {
        Rule::Sequence(Box::new(a.into()), Box::new(b.into()))
    }

    fn alt(a: impl Into<Rule>, b: impl Into<Rule>) -> Self {
        Rule::Alternative(Box::new(a.into()), Box::new(b.into()))
    }

    fn special(a: impl Into<Rule>, b: impl Into<Rule>) -> Self {
        Rule::Special(Box::new(a.into()), Box::new(b.into()))
    }

    fn from_str(s: &str) -> Self {
        Self::parse_alternative(s)
    }

    fn parse_alternative(s: &str) -> Self {
        let mut alternatives: Vec<_> = s.split(" | ").collect();

        let mut rule = Self::parse_sequence(alternatives.pop().unwrap());

        for alt in alternatives.into_iter().rev() {
            let subrule = Self::parse_sequence(alt);
            rule = Rule::alt(subrule, rule);
        }

        rule
    }

    fn parse_sequence(s: &str) -> Self {
        let mut sequence: Vec<_> = s.split_whitespace().collect();

        let mut rule = Self::parse_rule(sequence.pop().unwrap());

        for r in sequence.into_iter().rev() {
            let subrule = Self::parse_rule(r);
            rule = Rule::seq(subrule, rule);
        }

        rule
    }

    fn parse_rule(s: &str) -> Self {
        match s {
            "\"a\"" => Rule::Char(b'a'),
            "\"b\"" => Rule::Char(b'b'),
            _ => Rule::SubRule(s.parse().unwrap()),
        }
    }

    fn does_match_str_completely(&self, s: &[u8], ruleset: &HashMap<u8, Rule>) -> bool {
        self.does_match_str(s, ruleset)
            .map(|n| s.len() == n)
            .unwrap_or(false)
    }

    fn does_match_str(&self, s: &[u8], ruleset: &HashMap<u8, Rule>) -> Option<usize> {
        match self {
            Rule::Char(ch) => {
                if s.starts_with(&[*ch]) {
                    Some(1)
                } else {
                    None
                }
            }
            Rule::SubRule(r) => ruleset[r].does_match_str(s, ruleset),
            Rule::Alternative(a, b) => a
                .does_match_str(s, ruleset)
                .or_else(|| b.does_match_str(s, ruleset)),
            Rule::Sequence(a, b) => a
                .does_match_str(s, ruleset)
                .and_then(|n| b.does_match_str(&s[n..], ruleset).map(|nsub| nsub + n)),
            Rule::Special(a, b) => a.match_one_or_more(s, ruleset).and_then(|na| {
                b.match_one_or_more(&s[na..], ruleset)
                    .filter(|nb| nb < &na)
                    .map(|nb| na + nb)
            }),
        }
    }

    fn match_one_or_more(&self, s: &[u8], ruleset: &HashMap<u8, Rule>) -> Option<usize> {
        let mut n = 0;
        loop {
            match self.does_match_str(&s[n..], ruleset) {
                Some(nsub) => n += nsub,
                None => break,
            }
        }

        if n == 0 {
            None
        } else {
            Some(n)
        }
    }
}
