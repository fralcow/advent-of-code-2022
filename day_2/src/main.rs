fn main() {
    let rounds = std::fs::read_to_string("data.txt");

    let rounds = match rounds {
        Ok(m) => m,
        _ => {
            println!("failed to read the data from data.txt");
            return;
        }
    };

    let rounds: Vec<&str> = rounds.split("\n").collect();

    let score: i32 = rounds
        .clone()
        .into_iter()
        .filter(|round| round.len() > 0)
        .map(|round| {
            let round: Vec<&str> = round.split(" ").collect();
            let opponent = round[0];
            let me = round[1];
            return round_score(opponent, me);
        })
        .sum();

    println!("score: {:?}", score);

    let score_v2: i32 = rounds
        .into_iter()
        .filter(|round| round.len() > 0)
        .map(|round| {
            let round: Vec<&str> = round.split(" ").collect();
            let opponent = round[0];
            let me = round[1];
            return round_score_v2(opponent, me);
        })
        .sum();

    println!("score_v2: {:?}", score_v2);
}

fn round_score(opponent: &str, me: &str) -> i32 {
    let mut score: i32 = 0;

    score += match me {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };

    score += match opponent {
        "A" => match me {
            "X" => 3,
            "Y" => 6,
            _ => 0,
        },
        "B" => match me {
            "Y" => 3,
            "Z" => 6,
            _ => 0,
        },
        "C" => match me {
            "Z" => 3,
            "X" => 6,
            _ => 0,
        },
        _ => 0,
    };

    return score;
}

// The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"
//
// The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:
//
//     In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
//     In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
//     In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
//
// Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.
fn round_score_v2(opponent: &str, me: &str) -> i32 {
    let mut score: i32 = 0;

    score += match me {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => 0,
    };

    score += match opponent {
        "A" => match me {
            "X" => 3,
            "Y" => 1,
            "Z" => 2,
            _ => 0,
        },
        "B" => match me {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        },
        "C" => match me {
            "X" => 2,
            "Y" => 3,
            "Z" => 1,
            _ => 0,
        },
        _ => 0,
    };

    return score;
}
