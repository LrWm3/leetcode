use std::env;
//
// 2020-06-07
//
// Usage
//   see help()
//
fn help() {
  println!(
    "
  2020-06-07
  You are given coins of different denominations and a total amount of money.
  Write a function to compute the number of combinations that make up that amount.
  You may assume that you have infinite number of each kind of coin.
  > Example 1:
  > 
  > Input: amount = 5, coins = [1, 2, 5]
  > Output: 4
  > Explanation: there are four ways to make up the amount:
  > 5=5
  > 5=2+2+1
  > 5=2+1+1+1
  > 5=1+1+1+1+1
  > 
  > Example 2:
  > 
  > Input: amount = 3, coins = [2]
  > Output: 0
  > Explanation: the amount of 3 cannot be made up just with coins of 2.
  > Example 3:
  > 
  > Input: amount = 10, coins = [10] 
  > Output: 1
  Note:
  You can assume that
  0 <= amount <= 5000
  1 <= coin <= 5000
  the number of coins is less than 500
  the answer is guaranteed to fit into signed 32-bit integer
  Usage
  <program_name> [amount] (coin1 coin2 coin 3 .. coin N)"
  );
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  #[test]
  fn test_change_1() {
    let coins = vec![1, 2, 5];
    let amount = 5;
    assert_eq!(4, Solution::change(amount, coins));
  }

  #[test]
  fn test_change_2() {
    let coins = vec![2, 3, 5];
    let amount = 5;
    assert_eq!(2, Solution::change(amount, coins));
  }

  #[test]
  fn test_change_w_zero_amount() {
    let coins = vec![1, 2, 3];
    let amount = 0;
    assert_eq!(1, Solution::change(amount, coins));
  }

  #[test]
  fn test_change_w_zero_coins() {
    let coins: Vec<i32> = vec![];
    let amount = 1;
    assert_eq!(0, Solution::change(amount, coins));
  }

  #[test]
  fn test_change_w_amount_as_one() {
    let coins: Vec<i32> = vec![2, 3];
    let amount = 1;
    assert_eq!(0, Solution::change(amount, coins));
  }

  #[test]
  fn test_change_w_amount_as_one_and_coin_as_one() {
    let coins: Vec<i32> = vec![1, 2, 3];
    let amount = 1;
    assert_eq!(1, Solution::change(amount, coins));
  }
}

struct Solution {}

impl Solution {
  pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let extent = amount + 1;
    let mut num_ways_for_amt: Vec<i32> = vec![0; extent as usize];

    // when coin - amt_idx = 0, add 1
    num_ways_for_amt[0] = 1;

    // process bottom up to use previous num ways to compute latter
    for coin in coins {
      for amt_idx in coin..extent {
        num_ways_for_amt[amt_idx as usize] += num_ways_for_amt[(amt_idx - coin) as usize];
      }
    }
    return num_ways_for_amt[amount as usize];
  }
}

fn main() -> Result<(), ()> {
  let args: Vec<String> = env::args().collect();
  match args.len() {
    // no arguments passed
    1 => {
      help();
      println!("No arguments provided!");
      return Result::Err(());
    }
    // one argument passed
    2 => {
      let amount = args[0]
        .parse::<i32>()
        .expect("Failed to parse amount from first argument, did you provide a number?");
      let coins = vec![];
      println!(
        "amount={}, coins={:?}, coin_permutations={}",
        amount,
        coins,
        Solution::change(amount, coins.to_vec())
      );
      return Result::Ok(());
    }
    // all the other cases
    _ => {
      let amount = args[0]
        .parse::<i32>()
        .expect("Failed to parse amount from first argument, did you provide a number?");
      let coins = vec![];
      println!(
        "amount={}, coins={:?}, coin_permutations={}",
        amount,
        coins,
        Solution::change(amount, coins.to_vec())
      );
      return Result::Ok(());
    }
  }
}
