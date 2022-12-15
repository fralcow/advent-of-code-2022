mod input;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{one_of, space1, u64},
    combinator::{all_consuming, map, value},
    multi::separated_list1,
    sequence::{preceded, tuple},
    Finish, IResult,
};

#[derive(Debug, Clone)]
struct Monkey {
    items_inspected: u64,
    items: Vec<u64>,
    operation: Operation,
    divisor: u64,
    receiver_if_true: usize,
    receiver_if_false: usize,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Term, Term),
    Mul(Term, Term),
}

impl Operation {
    fn eval(self, old: u64) -> u64 {
        match self {
            Operation::Add(l, r) => l.eval(old) + r.eval(old),
            Operation::Mul(l, r) => l.eval(old) * r.eval(old),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Term {
    Old,
    Constant(u64),
}

impl Term {
    fn eval(self, old: u64) -> u64 {
        match self {
            Term::Old => old,
            Term::Constant(c) => c,
        }
    }
}

fn parse_term(i: &str) -> IResult<&str, Term> {
    alt((value(Term::Old, tag("old")), map(u64, Term::Constant)))(i)
}

fn parse_operation(i: &str) -> IResult<&str, Operation> {
    let (i, (l, op, r)) = preceded(
        tag("new = "),
        tuple((
            parse_term,
            preceded(space1, one_of("*+")),
            preceded(space1, parse_term),
        )),
    )(i)?;
    let op = match op {
        '*' => Operation::Mul(l, r),
        '+' => Operation::Add(l, r),
        _ => unreachable!(),
    };
    Ok((i, op))
}

fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    let (i, _) = tuple((tag("Monkey "), u64, tag(":\n")))(i)?;

    let (i, (_, _, items, _)) = tuple((
        space1,
        tag("Starting items: "),
        separated_list1(tag(", "), u64),
        tag("\n"),
    ))(i)?;

    let (i, (_, _, operation, _)) =
        tuple((space1, tag("Operation: "), parse_operation, tag("\n")))(i)?;
    let (i, (_, _, divisor, _)) = tuple((space1, tag("Test: divisible by "), u64, tag("\n")))(i)?;
    let (i, (_, _, receiver_if_true, _)) = tuple((
        space1,
        tag("If true: throw to monkey "),
        map(u64, |x| x as usize),
        tag("\n"),
    ))(i)?;
    let (i, (_, _, receiver_if_false, _)) = tuple((
        space1,
        tag("If false: throw to monkey "),
        map(u64, |x| x as usize),
        tag("\n"),
    ))(i)?;

    Ok((
        i,
        Monkey {
            items_inspected: 0,
            items,
            operation,
            divisor,
            receiver_if_true,
            receiver_if_false,
        },
    ))
}

fn parse_all_monkeys(i: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(tag("\n"), parse_monkey)(i)
}

fn do_round(monkeys: &mut [Monkey]) {
    let num_monkeys = monkeys.len();

    #[allow(clippy::needless_range_loop)]
    for i in 0..num_monkeys {
        let mc;

        {
            let monkey = &mut monkeys[i];
            mc = monkey.clone();
            monkey.items_inspected += mc.items.len() as u64;
        }

        for mut item in mc.items.iter().copied() {
            item = mc.operation.eval(item);
            item /= 3;
            if item % mc.divisor == 0 {
                monkeys[mc.receiver_if_true].items.push(item);
            } else {
                monkeys[mc.receiver_if_false].items.push(item);
            }
        }
        monkeys[i].items.clear();
    }
}

fn main() {
    let mut input = input::REAL_INPUT.to_string();
    input.push('\n');

    let monkeys = all_consuming(parse_all_monkeys)(&input).finish().unwrap().1;

    let mut monkeys = monkeys;
    for i in 0..20 {
        println!("Round {}", i + 1);
        do_round(&mut monkeys);
        for monkey in &monkeys {
            println!("{monkey:?}");
        }
    }

    let mut all_inspect_counts = monkeys
        .iter()
        .map(|m| m.items_inspected)
        .collect::<Vec<_>>();
    all_inspect_counts.sort_by_key(|&c| std::cmp::Reverse(c));
    let monkey_business = all_inspect_counts.into_iter().take(2).product::<u64>();
    dbg!(monkey_business);
}
