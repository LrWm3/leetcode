#[cfg(test)]
mod tests {

    #[test]
    fn test_reconstruct_queue() {
        assert_eq!(1 + 2, 3);
    }
}

struct Solution { }

#[allow(dead_code)]
impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

      let height_idx: usize = 0;
      let taller_idx: usize = 1;

      let mut people_copy = people.clone();
      
      // let mut people = vec![vec![7,0], vec![4,4], vec![7,1], vec![5,0], vec![6,1], vec![5,2]];
  
      // Contains heights we've inserted already, sorted asc
      let people_len = people_copy.len();
      let people_height_insert_desc: Vec<i32> = vec![std::i32::MAX];
      let people_reconstruct: Vec<Vec<i32>> = vec![vec![]; people_len];
 
      let most_taller_in_front = people_copy.iter()
        .max_by(|a, b| (a[taller_idx]).cmp(
                      &(b[taller_idx])))
        .unwrap_or(&vec![0,0])[taller_idx];      

      // Create people stacks
      let mut people_stacks: Vec<Option<Vec<Vec<i32>>>> = vec![None; most_taller_in_front as usize];

      // Sort people provided into a stack where grouped by num-taller and heights are sub-sorted descending
      people_copy.sort_unstable_by(|a, b| (a[taller_idx], b[height_idx]).cmp(&(b[taller_idx], a[height_idx])));

      // Push people onto the appropriate people stacks
      while let Some(person) = people_copy.pop() {
        let stack_idx: usize = person[taller_idx] as usize;
        let people_stack_opt: &mut Option<Vec<Vec<i32>>> = &mut people_stacks[stack_idx];
        let people_stack: &mut Vec<Vec<i32>> = people_stack_opt.get_or_insert_with(|| vec![]);
        people_stack.push(person);
      }

      println!("people[] {:?}", people_copy);
      println!("people_copy[] {:?}", people_copy);
      println!("people_stacks[] {:?}", people_stacks);
      println!("most_taller_in_front {}", most_taller_in_front);
      println!("people_height_insert_desc: {:?}", people_height_insert_desc);
      println!("people_reconstruct:: {:?}", people_reconstruct);


      // Given the number of heights so far, iterate 
      // For person in 0..people_len
      //   For target_taller in people_height_insert_desc..0;
      //     peek from people_stacks; if none continue, if value
      //       if minimum_height lt 'people_height_insert_desc[target_taller]'
      //         remove / pop person from people_stack[target_taller]
      //         insert person.height in front of 'people_height_insert_desc[target_taller]'
      //            e.g. if 'target_taller' is '0', insert at idx '1'
      //         push person onto 'person_reconstruct'
      //      if no people met condition, the list is invalid, which sucks
      return people_copy;
    }
}

fn main() {
    println!("Hello, world!");
}
