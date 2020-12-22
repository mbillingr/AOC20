use common::input::Input;
use common::itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = Input::from_file("data/day22-input.txt");

    let mut players = input
        .iter_blocks()
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(str::parse)
                .map(Result::unwrap)
                .collect::<VecDeque<_>>()
        })
        //.inspect(|x| println!("{:?}", x))
        .collect::<Vec<_>>();

    while winner(&players).is_none() {
        round(&mut players);
    }
    let winning_player = winner(&players).unwrap();

    println!("Part 1: {}", score(players[winning_player].iter()));

    let players = input
        .iter_blocks()
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(str::parse)
                .map(Result::unwrap)
                .collect::<VecDeque<_>>()
        })
        //.inspect(|x| println!("{:?}", x))
        .collect::<Vec<_>>();

    let mut game = RecursiveGame::new(players);
    let winner = game.play();

    println!("Part 2: {}", score(game.players[winner].iter()));
}

fn score<'a>(player: impl DoubleEndedIterator<Item = &'a usize>) -> usize {
    player
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, value)| value * (i + 1))
        .sum()
}

fn winner<T: Deck<usize>>(players: &Vec<T>) -> Option<usize> {
    let mut potential_winners = players
        .iter()
        .enumerate()
        .filter(|(_, deck)| !deck.is_empty());

    let winner = potential_winners.next();

    winner.and_then(|(i, _)| {
        if potential_winners.next().is_none() {
            Some(i)
        } else {
            None
        }
    })
}

fn round<T: Deck<usize>>(players: &mut Vec<T>) {
    let mut top_cards: Vec<_> = players.iter_mut().map(Deck::draw_top).collect();
    let winner = top_cards.iter().position_max().unwrap();
    top_cards.sort();
    while let Some(card) = top_cards.pop() {
        players[winner].put_bottom(card);
    }
}

trait Deck<T> {
    fn is_empty(&self) -> bool;
    fn draw_top(&mut self) -> T;
    fn put_bottom(&mut self, card: T);
}

impl<T> Deck<T> for VecDeque<T> {
    fn is_empty(&self) -> bool {
        VecDeque::is_empty(self)
    }

    fn draw_top(&mut self) -> T {
        self.pop_front().unwrap()
    }

    fn put_bottom(&mut self, card: T) {
        self.push_back(card)
    }
}

struct RecursiveGame {
    states: HashSet<Vec<VecDeque<usize>>>,
    winner_cache: HashMap<Vec<VecDeque<usize>>, usize>,
    players: Vec<VecDeque<usize>>,
}

impl RecursiveGame {
    fn new(players: Vec<VecDeque<usize>>) -> Self {
        RecursiveGame {
            states: HashSet::new(),
            winner_cache: HashMap::new(),
            players,
        }
    }

    fn play(&mut self) -> usize {
        if let Some(w) = self.winner_cache.get(&self.players) {
            return *w;
        }

        loop {
            if let Some(win) = self.round() {
                self.winner_cache.insert(self.players.clone(), win);
                return win;
            }
        }
    }

    fn round(&mut self) -> Option<usize> {
        if !self.states.insert(self.players.clone()) {
            return Some(0);
        }

        let mut top_cards: Vec<_> = self.players.iter_mut().map(Deck::draw_top).collect();

        let round_winner = if (0..self.players.len()).all(|i| self.players[i].len() >= top_cards[i])
        {
            let sub_players: Vec<VecDeque<_>> = self
                .players
                .iter()
                .zip(&top_cards)
                .map(|(player, card)| player.iter().copied().take(*card).collect())
                .collect();
            RecursiveGame::new(sub_players).play()
        } else {
            top_cards.iter().position_max().unwrap()
        };

        self.players[round_winner].put_bottom(top_cards.swap_remove(round_winner));
        self.players[round_winner].put_bottom(top_cards.pop().unwrap());

        winner(&self.players)
    }
}
