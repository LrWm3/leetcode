use std::env;
//
// 2020-06-09
//
// Usage
//   see help()
//
fn help(args: Vec<String>) {
  println!(
    "
    2020-06-09
    url: https://leetcode.com/explore/featured/card/june-leetcoding-challenge/540/week-2-june-8th-june-14th/3355/

    Problem

    Given a string s and a string t, check if s is subsequence of t.

    A subsequence of a string is a new string which is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (ie, \"ace\" is a subsequence of \"abcde\" while \"aec\" is not).

    Follow up:
    If there are lots of incoming S, say S1, S2, ... , Sk where k >= 1B, and you want to check one by one to see if T has its subsequence. In this scenario, how would you change your code?

    Credits:
    Special thanks to @pbrother for adding this problem and creating all test cases.

    Example 1:

    > Input: s = \"abc\", t = \"ahbgdc\"
    > Output: true

    Example 2:

    > Input: s = \"axc\", t = \"ahbgdc\"
    > Output: false

    Note:

    * 0 <= s.length <= 100
    * 0 <= t.length <= 10^4
    * Both strings consists only of lowercase characters.

    Usage
        Supply the sub-string and string as arguments.
        {} [substring] [string]
",
    args[0]
  );
}

struct Solution {}

impl Solution {
  pub fn is_subsequence(s: String, t: String) -> bool {
    let slen = s.len();
    if s.len() == 0 {
      return true;
    }

    let sb = s.as_bytes();
    let tb = t.as_bytes();
    let mut ii = 0;
    for t in tb {
      if &sb[ii] == t {
        ii += 1;
        if ii == slen {
          return true;
        }
      }
    }
    return ii == slen;
  }
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::Solution;

  #[test]
  fn test_0() {
    let s = String::from("abc");
    let t = String::from("ahbgdc");
    assert_eq!(true, Solution::is_subsequence(t, s));
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
      let s = args[0].to_string();
      let t = args[1].to_string();
      println!("s={}, t={}", &s, &t);
      println!("is_subsequence={}", Solution::is_subsequence(s, t));
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
