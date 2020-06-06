//
// 2020-06-06
//
// Suppose you have a random list of people standing in a queue. Each person is described by a pair of integers (h, k), where h is the height of the person and k is the number of people in front of this person who have a height greater than or equal to h. Write an algorithm to reconstruct the queue.
//
// Note:
// The number of people is less than 1,100.
//
// Example
//
// Input:
// [[7,0], [4,4], [7,1], [5,0], [6,1], [5,2]]
//
// Output:
// [[5,0], [7,0], [5,2], [6,1], [4,4], [7,1]]
use std::cmp;

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  #[test]
  fn test_reconstruct_queue() {
    let people = vec![
      vec![7, 0],
      vec![4, 4],
      vec![7, 1],
      vec![5, 0],
      vec![6, 1],
      vec![5, 2],
    ];
    let people_expected = vec![
      vec![5, 0],
      vec![7, 0],
      vec![5, 2],
      vec![6, 1],
      vec![4, 4],
      vec![7, 1],
    ];
    let people_actual = Solution::reconstruct_queue(people);
    assert_eq!(people_actual, people_expected);
  }

  #[test]
  fn test_reconstruct_empty_queue() {
    let people: Vec<Vec<i32>> = vec![];
    let people_expected: Vec<Vec<i32>> = vec![];
    let people_actual = Solution::reconstruct_queue(people);
    assert_eq!(people_actual, people_expected);
  }

  #[test]
  fn test_reconstruct_one_in_queue() {
    let people = vec![vec![7, 0]];
    let people_expected = vec![vec![7, 0]];
    let people_actual = Solution::reconstruct_queue(people);
    assert_eq!(people_actual, people_expected);
  }
}

struct Solution {}

#[allow(dead_code)]
impl Solution {
  pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if people.len() == 0 || people.len() == 1 {
      return people;
    }

    let height_idx: usize = 0;
    let taller_idx: usize = 1;

    let mut people_copy = people.clone();

    // Contains heights we've inserted already, sorted asc
    let people_len = people_copy.len();
    let mut peoples_height_inserted_desc: Vec<i32> = vec![std::i32::MAX];
    peoples_height_inserted_desc.reserve(people_len + 1);

    let mut people_reconstruct: Vec<Vec<i32>> = vec![];
    people_reconstruct.reserve(people_len);

    // We want to have room for the 0 -> most_people_in_front + 1.
    let people_stack_size: usize = (people_copy
      .iter()
      .max_by(|a, b| (a[taller_idx]).cmp(&(b[taller_idx])))
      .unwrap_or(&vec![0, 0])[taller_idx]
      + 1) as usize;

    // Create people stacks
    let mut people_stacks: Vec<Option<Vec<Vec<i32>>>> = vec![None; people_stack_size];

    // Sort people provided into a stack where grouped by num-taller and heights are sub-sorted descending
    people_copy
      .sort_unstable_by(|a, b| (a[taller_idx], a[height_idx]).cmp(&(b[taller_idx], b[height_idx])));

    // Push people onto the appropriate people stacks
    while let Some(person) = people_copy.pop() {
      let stack_idx: usize = person[taller_idx] as usize;
      let people_stack_opt: &mut Option<Vec<Vec<i32>>> = &mut people_stacks[stack_idx];
      let people_stack: &mut Vec<Vec<i32>> = people_stack_opt.get_or_insert_with(|| vec![]);
      people_stack.push(person);
    }

    for _person_idx in 0..people_len {
      let peoples_height_inserted_len =
        cmp::min(peoples_height_inserted_desc.len(), people_stacks.len());

      // Start with the tallest and work our way backwards
      for target_height in (0..peoples_height_inserted_len).rev() {
        let person_stack = &mut people_stacks[target_height];
        if person_stack
          .as_ref()
          .and_then(|ps| ps.last())
          .map_or(false, |person_small| {
            let person_height = person_small[height_idx];
            if person_height <= peoples_height_inserted_desc[target_height] {
              peoples_height_inserted_desc.insert(target_height + 1, person_height);
              people_reconstruct.push(person_small.to_vec());
              return true;
            }
            return false;
          })
        {
          person_stack.as_mut().unwrap().pop();
          break;
        }
      }
    }
    return people_reconstruct;
  }
}

fn main() {
  println!("Hello, world!");
}
