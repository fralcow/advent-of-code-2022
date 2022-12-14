use std::ops::Range;

type PairRange = Vec<(Range<usize>, Range<usize>)>;

fn main() {
    let pairs = std::fs::read_to_string("input.txt");

    let pairs = match pairs {
        Ok(m) => m,
        _ => {
            println!("failed to read the data from input.txt");
            return;
        }
    };
    let _pairs = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    let pairs: Vec<&str> = pairs.split("\n").filter(|line| !line.is_empty()).collect();

    let pairs_range: PairRange = pairs
        .iter()
        .map(|pair| {
            let (left, right) = match pair.split_once(',') {
                Some(v) => v,
                _ => return None,
            };

            let left_range = match parse_range(left) {
                Ok(v) => v,
                Err(_) => return None,
            };

            let right_range = match parse_range(right) {
                Ok(v) => v,
                Err(_) => return None,
            };

            return Some((left_range, right_range));
        })
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .collect();

    // problem 1
    let number_contains_whole: usize = pairs_range
        .iter()
        .cloned()
        .map(|pair| {
            let left_range = pair.0;
            let right_range = pair.1;

            let mut to_return: Option<usize> = Some(0);
            if (left_range.start <= right_range.start && left_range.end >= right_range.end)
                || (right_range.start <= left_range.start && right_range.end >= left_range.end)
            {
                to_return = Some(1);
            };

            return to_return;
        })
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .sum();

    println!("number_contains_whole: {:?}", number_contains_whole);

    // problem 2
    let number_contains_partial: usize = pairs_range
        .iter()
        .map(|pair| {
            let left_range = &pair.0;
            let right_range = &pair.1;

            let mut to_return: Option<usize> = Some(0);

            // left_start .. right_start .. left_end
            if (left_range.start <= right_range.start && right_range.start <= left_range.end) ||
            // left_start .. right_end .. left_end
             (left_range.start <= right_range.end && right_range.end <= left_range.end) ||
            // right_start .. left_start .. right_end
             (right_range.start <= left_range.start && left_range.start <= right_range.end) ||
            // right_start .. left_end .. right_end
             (right_range.start <= left_range.end && left_range.end <= right_range.end)
            {
                to_return = Some(1);
            };

            return to_return;
        })
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .sum();

    println!("number_contains_partial: {:?}", number_contains_partial);
}
fn parse_range(range: &str) -> Result<Range<usize>, &'static str> {
    let (start, end) = match range.split_once('-') {
        Some(v) => v,
        None => {
            return Err("Failed to parse range");
        }
    };

    let start: usize = start.parse().unwrap();
    let end: usize = end.parse().unwrap();

    return Ok(Range { start, end });
}
