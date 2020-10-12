use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
struct Score {
    matches_played: usize,
    wins: usize,
    draws: usize,
    loses: usize,
    points: usize,
}

impl Score {
    fn new() -> Self {
        Score {
            matches_played: 0,
            wins: 0,
            draws: 0,
            loses: 0,
            points: 0,
        }
    }

    fn play(&mut self) {
        self.matches_played += 1;
    }

    fn win(&mut self) {
        self.play();
        self.wins += 1;
        self.points += 3;
    }

    fn draw(&mut self) {
        self.play();
        self.draws += 1;
        self.points += 1;
    }

    fn lose(&mut self) {
        self.play();
        self.loses += 1;
    }
}

pub fn tally(input: &str) -> String {
    let mut output = format!(
        "{:30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
        "Team", "MP", "W", "D", "L", "P"
    );
    let mut scores = HashMap::new();

    for line in input.lines() {
        let parts = line.split(';').collect::<Vec<&str>>();
        let (team1, team2, match_result) = (parts[0], parts[1], parts[2]);

        {
            let team1_entry = scores.entry(team1).or_insert(Score::new());

            match match_result {
                "win" => team1_entry.win(),
                "draw" => team1_entry.draw(),
                "loss" => team1_entry.lose(),
                _ => panic!("Invalid match result!"),
            }
        }
        {
            let team2_entry = scores.entry(team2).or_insert(Score::new());

            match match_result {
                "win" => team2_entry.lose(),
                "draw" => team2_entry.draw(),
                "loss" => team2_entry.win(),
                _ => panic!("Invalid match result!"),
            }
        }
    }

    let mut scores = scores.iter().collect::<Vec<_>>();
    scores.sort_by(|&(a_name, a_score), &(b_name, b_score)| {
        match b_score.points.cmp(&a_score.points) {
            Ordering::Equal => a_name.cmp(b_name),
            other => other,
        }
    });

    for (name, score) in scores {
        output.push_str(&format!(
            "\n{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            name, score.matches_played, score.wins, score.draws, score.loses, score.points
        ))
    }

    output
}
