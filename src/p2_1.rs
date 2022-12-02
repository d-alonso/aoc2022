use crate::Result;

enum RPS {
    Rock,
    Paper,
    Scissors,
}

struct OponnentPlay(RPS);
struct PlayerPlay(RPS);


// The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors
// The second column, you reason, must be what you should play in response: X for Rock, Y for Paper,
// and Z for Scissors

impl std::str::FromStr for OponnentPlay {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self(RPS::Rock)),
            "B" => Ok(Self(RPS::Paper)),
            "C" => Ok(Self(RPS::Scissors)),
            _ => Err(format!("Invalid rock/paper/scissors '{}'", s)),
        }
    }
}

impl std::str::FromStr for PlayerPlay {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self(RPS::Rock)),
            "Y" => Ok(Self(RPS::Paper)),
            "Z" => Ok(Self(RPS::Scissors)),
            _ => Err(format!("Invalid rock/paper/scissors '{}'", s)),
        }
    }
}

// Your total score is the sum of your scores for each round. The score for a single round is the score for
// the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome
// of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

fn round_result_score(opp: &OponnentPlay, player: &PlayerPlay) -> usize {
    match opp.0 {
        RPS::Rock => match player.0 {
            RPS::Rock => 3,
            RPS::Paper => 6,
            RPS::Scissors => 0,
        },
        RPS::Paper => match player.0 {
            RPS::Rock => 0,
            RPS::Paper => 3,
            RPS::Scissors => 6,
        },
        RPS::Scissors => match player.0 {
            RPS::Rock => 6,
            RPS::Paper => 0,
            RPS::Scissors => 3,
        },
    }
}

fn player_play_score(player: &PlayerPlay) -> usize {
    match player.0 {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}

fn round_score(opp: &OponnentPlay, player: &PlayerPlay) -> usize {
    round_result_score(opp, player) + player_play_score(player)
}

// What would your total score be if everything goes exactly according to your strategy guide?
fn p(input: &str) -> Result<usize> {
    let plays = input
        .trim()
        .lines()
        .map(|round: &str| -> Result<(OponnentPlay, PlayerPlay)> {
            // Split at the white space
            let (opponent_str, player_str) = (&round[0..1], &round[2..]);
            Ok((
                opponent_str.parse::<OponnentPlay>()?,
                player_str.parse::<PlayerPlay>()?,
            ))
        })
        .collect::<Result<Vec<_>>>()?; // Bubble up parse errors
    Ok(plays
        .into_iter()
        .map(|(opponent_play, player_play)| round_score(&opponent_play, &player_play))
        .sum())
}

#[test]
fn simple() {
    let input = r#"A Y
B X
C Z"#;
    assert!(matches!(dbg!(p(input)), Ok(15)));
}

#[test]
fn problem() {
    let input = include_str!("../inputs/p2.txt");
    assert!(matches!(dbg!(p(input)), Ok(12740)));
}
