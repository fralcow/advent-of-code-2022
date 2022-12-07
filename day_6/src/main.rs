use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt");

    let mut input = match input {
        Ok(m) => m,
        _ => {
            println!("failed to read the data from input.txt");
            return;
        }
    };

    // remove the newline
    input.pop();

    let input: Vec<char> = input.chars().collect();

    let mut index = 0;
    for w in input.windows(4) {
        let mut set: HashSet<char> = HashSet::new();

        w.iter().for_each(|&letter| {
            _ = set.insert(letter);
        });

        if set.len() == 4 {
            println!("index: {:?}", index + 4); // + 4 because we're looking for the index of the
                                                // last marker character
            return;
        }

        index += 1;
    }
}
