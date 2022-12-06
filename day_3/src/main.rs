use std::collections::HashSet;

fn main() {
    let rucksacks = std::fs::read_to_string("input.txt");

    let rucksacks = match rucksacks {
        Ok(m) => m,
        _ => {
            println!("failed to read the data from input.txt");
            return;
        }
    };

    let rucksacks: Vec<&str> = rucksacks
        .split("\n")
        .filter(|line| !line.is_empty())
        .collect();
    let mut dupes: Vec<char> = vec![];

    for rucksack in rucksacks.clone() {
        let set_left: HashSet<char, _> =
            HashSet::<char>::from_iter(rucksack[0..rucksack.len() / 2].chars());
        let set_right: HashSet<char, _> =
            HashSet::<char>::from_iter(rucksack[rucksack.len() / 2..rucksack.len()].chars());

        let dupe = set_left
            .intersection(&set_right)
            .cloned()
            .filter(|d| d.is_ascii_alphabetic())
            .next();

        match dupe {
            Some(d) => dupes.push(d),
            None => (),
        };
    }

    let priorities: usize = dupes
        .into_iter()
        .map(|dupe| {
            return calculate_priority(dupe);
        })
        .sum();

    println!("priorities 1: {:?}", priorities);

    // Problem 2
    let groups = rucksacks.chunks(3);

    let mut priorities: usize = 0;

    for g in groups {
        let set_1 = HashSet::<char>::from_iter(g[0].chars());
        let set_2 = HashSet::<char>::from_iter(g[1].chars());
        let set_3 = HashSet::<char>::from_iter(g[2].chars());

        let dupe: HashSet<char> = set_1.intersection(&set_2).cloned().collect();
        let dupe: HashSet<char> = dupe.intersection(&set_3).cloned().collect();

        priorities += calculate_priority(dupe.into_iter().next().unwrap());
    }

    println!("priorities 2: {:?}", priorities);
}

fn calculate_priority(letter: char) -> usize {
    let letter: usize = letter as usize;
    if letter >= 97 && letter <= 122 {
        return letter - (97 - 1);
    } else if letter >= 65 && letter <= 90 {
        return letter - (65 - 27);
    }

    return 0;
}
