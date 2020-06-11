fn main() {
  // let mut numbers: Vec<i32> = vec![0,2,1,2,2,0,1];
  // let mut numbers: Vec<i32> = vec![2,0,2,1,1,0];
  let mut numbers: Vec<i32> = vec![1, 0, 0];
  Solution::sort_colors(&mut numbers);
  println!("{:?}", numbers);
}

struct Solution {}

impl Solution {
  pub fn sort_colors(nums: &mut Vec<i32>) {
    let len: usize = nums.len();
    let last: usize = len - 1;
    let mut white_target: usize = 0;
    let mut current_idx: usize = 0;
    let mut blue_target: usize = last;

    for ii in 0..len {
      let idx = current_idx;
      println!(
        "r{} - (pre) idx={}, color={}, nums={:?}",
        ii, idx, nums[idx], nums
      );
      match nums[idx] {
        0 => {
          if idx != white_target {
            nums.swap(idx, white_target);
          }
          current_idx += 1;
          white_target += 1;
        }
        1 => {
          // nums.swap(idx, current_idx);
          current_idx += 1;
        }
        2 => {
          if idx == blue_target {
            break;
          }
          nums.swap(idx, blue_target);
          blue_target -= 1;
        }
        _ => println!("Invalid number received! only [0,1,2] are valid"),
      }
      println!(
        "r{} - (pst) idx={}, color={}, nums={:?}",
        ii, idx, nums[idx], nums
      );
    }
  }
}
