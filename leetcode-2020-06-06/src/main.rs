#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test_reconstruct_queue() {
        let people = vec![vec![7,0], vec![4,4], vec![7,1], vec![5,0], vec![6,1], vec![5,2]];
        let people_expected = vec![vec![5,0], vec![7,0], vec![5,2], vec![6,1], vec![4,4], vec![7,1]];
        let people_actual = Solution::reconstruct_queue(people);
        assert_eq!(people_actual, people_expected);
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
      let mut peoples_height_inserted_desc: Vec<i32> = vec![std::i32::MAX];
      peoples_height_inserted_desc.reserve(people_copy.len() + 1);

      let mut people_reconstruct: Vec<Vec<i32>> = vec![vec![]; people_len];

      // We want to have room for the 0 -> most_people_in_front + 1.
      let people_stack_size: usize = (people_copy.iter()
        .max_by(|a, b| (a[taller_idx]).cmp(
                      &(b[taller_idx])))
        .unwrap_or(&vec![0,0])[taller_idx] + 1) as usize;      

      // Create people stacks
      let mut people_stacks: Vec<Option<Vec<Vec<i32>>>> = vec![None; people_stack_size];

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
      println!("people_stack_size {}", people_stack_size);
      println!("peoples_height_inserted_desc: {:?}", peoples_height_inserted_desc);
      println!("people_reconstruct:: {:?}", people_reconstruct);


      for person_idx in 0..people_len {
        println!("finding person 1... {}", person_idx);
        let peoples_height_inserted_len = peoples_height_inserted_desc.len();

        // // Start with the tallest and work our way backwards
        // for target_height in peoples_height_inserted_len..0 {
        //   if people_stacks[target_height]
        //     .map(|person_stack| (person_stack, person_stack.iter().rev().next()))
        //     .filter(|person_stack_combo| person_stack_combo.1.unwrap()[height_idx] < peoples_height_inserted_desc[target_height])
        //     .as_ref()
        //     .map(|person_stack_combo| {
        //       let person_stack = person_stack_combo.0;
        //       let person = person_stack.pop().unwrap();
        //       let person_height = person[height_idx];
        //       peoples_height_inserted_desc.insert(target_height + 1, person_height);
        //       people_reconstruct.push(person);
        //       true
        //     }).unwrap_or(false) {
        //       break;
        //     }
        // }
        // Start with the tallest and work our way backwards
        for target_height in peoples_height_inserted_len..0 {
          let person_stack = people_stacks[target_height];
          if person_stack.unwrap().last().map_or(false, |person_small| {
            let person_height = person_small[height_idx];
            if person_height < peoples_height_inserted_desc[target_height] {
                person_stack.unwrap().pop();
                peoples_height_inserted_desc.insert(target_height + 1, person_height);
                people_reconstruct.push(person_small);
            }
          }) {
            break;
          }
        }
      }
      // Given the number of heights so far, iterate 
      // For person in 0..people_len
      //   For target_taller in peoples_height_inserted_desc..0;
      //     peek from people_stacks; if none continue, if value
      //       if minimum_height lt 'peoples_height_inserted_desc[target_taller]'
      //         remove / pop person from people_stack[target_taller] <- done
      //         insert person.height in front of 'peoples_height_inserted_desc[target_taller]' <- done
      //            e.g. if 'target_taller' is '0', insert at idx '1' 
      //         push person onto 'person_reconstruct' 
      //      if no people met condition, the list is invalid, which sucks
      return people_copy;
    }
}

fn main() {
    println!("Hello, world!");
}
