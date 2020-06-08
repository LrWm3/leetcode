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
  pub fn is_power_of_two(n: i32) -> bool {
    const POWS: [i32; 31] = [
      0x00000001, 0x00010000, 0x00000002, 0x00020000, 0x00000004, 0x00040000, 0x00000008,
      0x00080000, 0x00000010, 0x00100000, 0x00000020, 0x00200000, 0x00000040, 0x00400000,
      0x00000080, 0x00800000, 0x00000100, 0x01000000, 0x00000200, 0x02000000, 0x00000400,
      0x04000000, 0x00000800, 0x08000000, 0x00001000, 0x10000000, 0x00002000, 0x20000000,
      0x00004000, 0x40000000, 0x00008000,
    ];
    return POWS.contains(&n);
  }
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::Solution;

  #[test]
  fn test_0() {
    let amount = 0;
    assert_eq!(true, Solution::is_power_of_two(amount));
  }

  #[test]
  fn test_1() {
    let amount = 1;
    assert_eq!(true, Solution::is_power_of_two(amount));
  }

  #[test]
  fn test_2() {
    let amount = 2;
    assert_eq!(true, Solution::is_power_of_two(amount));
  }

  #[test]
  fn test_3() {
    let amount = 3;
    assert_eq!(false, Solution::is_power_of_two(amount));
  }

  #[test]
  fn test_4() {
    let amount = (2 as i32).pow(30);
    assert_eq!(false, Solution::is_power_of_two(amount));
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
      println!(
        "amount={}, is_power_of_two={}",
        amount,
        Solution::is_power_of_two(amount)
      );
      return Result::Ok(());
    }
    // all the other cases
    _ => {
      help(args);
      println!("Too many arguments provided!");
      return Result::Ok(());
    }
  }
}
