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

    let mut score: i32;

    let scores: Vec<i32> = rounds
        .into_iter()
        .filter(|round| round.len() > 0)
        .map(|round| {
            let round: Vec<&str> = round.split(" ").collect();
            let opponent = round[0];
            let me = round[1];
            return round_score(opponent, me);
        })
        .collect();

    println!("sum: {:?}", scores.into_iter().sum::<i32>());
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

    println!("opponent, me, score: {:?},{:?},{:?}", opponent, me, score);

    return score;
}
