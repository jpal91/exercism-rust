#![allow(unused)]
use std::collections::HashMap;
use std::hash::Hash;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Team {
    Alaskans,
    Badgers,
    Donkeys(String), // Because the tests changed spelling half way through?!
    Californians,
    None,
}

enum Results {
    Won,
    Tied,
    Lost,
}

#[derive(Default, PartialEq, Eq, Debug)]
struct Stats(u8, u8, u8, u8, u8);

struct Tournament {
    teams: HashMap<Team, Stats>,
}

impl Tournament {
    fn new() -> Self {
        Self {
            teams: HashMap::new()
        }
    }

    fn parse_matches(&mut self, inp: &str) {
        for line in inp.lines() {
            let results = line.split(';').collect::<Vec<_>>();

            let (team_a, team_b, res1, res2): (Team, Team, Results, Results) =
                match (results[0], results[1], results[2]) {
                    (a, b, "win") => (a.into(), b.into(), Results::Won, Results::Lost),
                    (a, b, "loss") => (a.into(), b.into(), Results::Lost, Results::Won),
                    (a, b, "draw") => (a.into(), b.into(), Results::Tied, Results::Tied),
                    _ => unreachable!()
                };
            
            self.teams
                .entry(team_a)
                .and_modify(|t| t.increment(&res1))
                .or_insert(Stats::from_result(res1));

            self.teams
                .entry(team_b)
                .and_modify(|t| t.increment(&res2))
                .or_insert(Stats::from_result(res2));
        }
    }

    fn results(self) -> String {
        let mut teams: Vec<(Stats, String)> = self.teams
            .into_iter()
            .map(|(t, s)| (s, t.to_string()))
            .collect();

        teams.sort();

        let mut res = String::from("Team                           | MP |  W |  D |  L |  P\n");

        for (stats, name) in teams {
            let mut tmp = name;
            let s = stats.to_string() + "\n";
            tmp += &" ".repeat(55 - (tmp.len() + s.len() - 1));
            res += &(tmp + &s);
        }

        res.pop();
        res
    }
}


impl Stats {
    fn from_result(result: Results) -> Self {
        let mut new = Self::default();
        new.increment(&result);

        new
    }
    
    fn increment(&mut self, result: &Results) {
        self.0 += 1;

        match result {
            Results::Won => {
                self.1 += 1;
                self.4 += 3;
            }
            Results::Tied => {
                self.2 += 1;
                self.4 += 1;
            }
            Results::Lost => self.3 += 1
        }
    }
}

impl ToString for Stats {
    fn to_string(&self) -> String {
        let mut res = String::new();

        for stat in [self.0, self.1, self.2, self.3, self.4] {
            let s = match stat {
                n if n >= 10 => format!("| {} ", n),
                n => format!("|  {} ", n)
            };
            res += &s
        };
        res.pop();
        res
    }
}

impl<'a> From<&'a str> for Team {
    fn from(value: &'a str) -> Self {
        match value {
            "Allegoric Alaskans" => Team::Alaskans,
            n @ ("Devestating Donkeys" | "Devastating Donkeys") => Team::Donkeys(n.to_string()),
            "Blithering Badgers" => Team::Badgers,
            "Courageous Californians" => Team::Californians,
            _ => Team::None,
        }
    }
}

impl ToString for Team {
    fn to_string(&self) -> String {
        let res = match self {
            Team::Alaskans => "Allegoric Alaskans",
            Team::Badgers => "Blithering Badgers",
            Team::Californians => "Courageous Californians",
            Team::Donkeys(n) => n,
            _ => ""
        };
        res.to_string()
    }
}


impl Ord for Stats {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.4.cmp(&self.4)
    }
}

impl PartialOrd for Stats {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn tally(match_results: &str) -> String {
    let mut tournament = Tournament::new();
    tournament.parse_matches(match_results);

    tournament.results()
}
