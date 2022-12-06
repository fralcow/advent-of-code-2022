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

    let rucksacks: Vec<&str> = rucksacks.split("\n").collect();
    let mut dupes: Vec<char> = vec![];

    for rucksack in rucksacks {
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

    println!("dupes: {:?}", dupes);

    let priorities: usize = dupes
        .into_iter()
        .map(|dupe| {
            let dupe = dupe as usize;
            if dupe >= 97 && dupe <= 122 {
                return dupe - (97 - 1);
            } else if dupe >= 65 && dupe <= 90 {
                return dupe - (65 - 27);
            }

            return 0;
        })
        .sum();

    println!("priorities: {:?}", priorities);
}
