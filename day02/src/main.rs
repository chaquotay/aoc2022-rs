use std::fs;
use std::str::FromStr;

trait Score {
    fn score(self) -> i32;
}

#[derive(Debug,Clone,Copy)]
enum RPC {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug,Clone,Copy)]
enum RoundResult {
    Victory,
    Defeat,
    Tie
}

impl RPC {
    fn play_against(self, opp: RPC) -> RoundResult {
        match (self, opp) {
            (RPC::Rock, RPC::Paper) => RoundResult::Victory,
            (RPC::Paper, RPC::Rock) => RoundResult::Defeat,
            (RPC::Rock, RPC::Scissors) => RoundResult::Defeat,
            (RPC::Scissors, RPC::Rock) => RoundResult::Victory,
            (RPC::Scissors, RPC::Paper) => RoundResult::Defeat,
            (RPC::Paper, RPC::Scissors) => RoundResult::Victory,
            _ => RoundResult::Tie
        }
    }
}

impl FromStr for RPC {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RPC::Rock),
            "B" => Ok(RPC::Paper),
            "C" => Ok(RPC::Scissors),
            "X" => Ok(RPC::Rock),
            "Y" => Ok(RPC::Paper),
            "Z" => Ok(RPC::Scissors),
            _ => Err(())
        }
    }
}



impl RoundResult {
    fn achive_against(self, opp: RPC) -> RPC {
        match (opp, self) {
            (RPC::Rock, RoundResult::Victory) => RPC::Paper,
            (RPC::Paper, RoundResult::Victory) => RPC::Scissors,
            (RPC::Scissors, RoundResult::Victory) => RPC::Rock,
            
            (x, RoundResult::Tie) => x,

            (RPC::Rock, RoundResult::Defeat) => RPC::Scissors,
            (RPC::Paper, RoundResult::Defeat) => RPC::Rock,
            (RPC::Scissors, RoundResult::Defeat) => RPC::Paper
        }
    }
}

impl FromStr for RoundResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RoundResult::Defeat),
            "Y" => Ok(RoundResult::Tie),
            "Z" => Ok(RoundResult::Victory),
            _ => Err(())
        }
    }
}

impl Score for RoundResult {
    fn score(self) -> i32 {
        match self {
            RoundResult::Victory => 6,
            RoundResult::Tie => 3,
            RoundResult::Defeat => 0,
        }
    }
}

impl Score for RPC {
    fn score(self) -> i32 {
        match self {
            RPC::Rock => 1,
            RPC::Paper => 2,
            RPC::Scissors => 3
        }
    }
}

impl Score for (RPC, RoundResult)
{
    fn score(self) -> i32 {
        self.0.score() + self.1.score()
    }
}

fn score<T>(s : T) -> i32
    where T : Score
{
    s.score()    
}

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();
    let mut rounds = Vec::<(RPC, RPC, RoundResult)>::new();

    for line in text.lines() {
        let opp = RPC::from_str(&line[0..1]);
        let me = RPC::from_str(&line[2..3]);
        let me_result = RoundResult::from_str(&line[2..3]);

        match (opp, me, me_result) {
            (Ok(opp), Ok(me), Ok(res)) => rounds.push((opp, me, res)),
            _ => println!("Urgs")
        };
    }

    let result1 : i32 = rounds.iter().map(|r| (r.1, r.0.play_against(r.1))).map(score).sum();
    let result2 : i32 = rounds.iter().map(|r| (r.2.achive_against(r.0), r.2)).map(score).sum();
    println!("Round 1: {}", result1);
    println!("Round 2: {}", result2);

    assert_eq!(14375, result1);
    assert_eq!(10274, result2);
}
