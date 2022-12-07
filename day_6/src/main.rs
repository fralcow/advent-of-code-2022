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

    // problem 1
    let problem_1_answer = find_marker_index(input.clone(), 4);
    println!("problem_1_answer: {:?}", problem_1_answer);

    // problem_2_answer
    let problem_2_answer = find_marker_index(input, 14);
    println!("problem_2_answer: {:?}", problem_2_answer);
}

fn find_marker_index(input: Vec<char>, window_size: usize) -> usize {
    let mut index: usize = 0;
    for w in input.windows(window_size) {
        let mut set: HashSet<char> = HashSet::new();

        w.iter().for_each(|&letter| {
            _ = set.insert(letter);
        });

        if set.len() == window_size {
            return index + window_size;
        }

        index += 1;
    }
    return 0;
}
