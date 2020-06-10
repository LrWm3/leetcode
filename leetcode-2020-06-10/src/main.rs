use std::env;
//
// 2020-06-10
//
// Usage
//   see help()
//

/**
 * Help
 */
fn help(args: Vec<String>) {
  println!(
    "
    2020-06-10
    url: https://leetcode.com/explore/featured/card/june-leetcoding-challenge/540/week-2-june-8th-june-14th/3356/

    Problem: Search Insert Position

    Given a sorted array and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

    You may assume no duplicates in the array.

    Example 1:

    Input: [1,3,5,6], 5
    Output: 2

    Example 2:

    Input: [1,3,5,6], 2
    Output: 1

    Example 3:

    Input: [1,3,5,6], 7
    Output: 4

    Example 4:

    Input: [1,3,5,6], 0
    Output: 0

    Note:

    No restriction on input values

    Usage
        Supply the sub-string and string as arguments.
        {} [substring] [string]
",
    args[0]
  );
}

/**
 * Implementation
 */
struct Solution {}

impl Solution {
  pub fn search_insert(s: Vec<i32>, t: i32) -> i32 {
    for ii in 0..s.len() {
      if s[ii] >= t {
        return ii as i32;
      }
    }
    return s.len() as i32;
  }
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::Solution;

  #[test]
  fn test_0() {
    let list = vec![1, 3, 5, 8];
    let target = 6;
    assert_eq!(3, Solution::search_insert(list, target));
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
      help(args);
      println!("Require two arguments!");
      return Result::Err(());
    }
    // two arguments passed
    3 => {
      // Method interface forces a copy
      let target: i32 = args[1]
        .parse::<i32>()
        .expect("Failed to parse i32 from argument one!");
      let list: Vec<i32> = args[2..]
        .iter()
        .map(|val| {
          val
            .parse::<i32>()
            .expect("Failed to parse one of the provided 'vals' for array")
        })
        .collect();
      println!("list={:?}, target={}", &list, &target);
      println!("search_insert={}", Solution::search_insert(list, target));
      return Result::Err(());
    }
    // all the other cases
    _ => {
      help(args);
      println!("Too many arguments provided!");
      return Result::Ok(());
    }
  }
}
