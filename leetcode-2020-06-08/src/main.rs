use std::env;
//
// 2020-06-08
//
// Usage
//   see help()
//
fn help(args: Vec<String>) {
    println!(
        "
    2020-06-08
    url: [add URL]

    Problem

    [add problem text]

    > [add examples]

    Note:

    [add notes]

    Usage
        [add usage]
        {} [amount] (coin1 coin2 coin3 .. coinN)
",
        args[0]
    );
}

struct Solution {}

impl Solution {
    // TODO - add interface
    pub fn change() -> i32 {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::change());
    }
}

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        // no arguments passed
        1 => {
            help(args);
            println!("No arguments provided!");
            return Result::Err(());
        }
        // one argument passed
        2 => {
            let amount = args[1].parse::<i32>().expect("Failed to parse 'amount'");
            let coins = vec![];
            println!(
                "amount={}, coins={:?}, coin_permutations={}",
                amount,
                coins,
                Solution::change()
            );
            return Result::Ok(());
        }
        // all the other cases
        _ => {
            let amount = args[1].parse::<i32>().expect("Failed to parse 'amount'");
            let coins: Vec<i32> = args[2..]
                .iter()
                .map(|coin| {
                    coin.parse::<i32>()
                        .expect("Failed to parse one of the provided 'coin'")
                })
                .collect();
            println!(
                "amount={}, coins={:?}, coin_permutations={}",
                amount,
                coins,
                Solution::change()
            );
            return Result::Ok(());
        }
    }
}
