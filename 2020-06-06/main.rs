impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

      let height_idx = 0;
      let taller_idx = 1;
      
      // let mut people = vec![vec![7,0], vec![4,4], vec![7,1], vec![5,0], vec![6,1], vec![5,2]];
  
      // Contains heights we've inserted already, sorted asc
      let people_len = people.len()
      let mut people_height_insert_desc: Vec<i32> = vec![i32::MAX];
      let mut people_reconstruct: Vec<Vec<i32>> = vec![vec![]; people_len];
 
      let most_taller_in_front = people.iter()
        .max_by(|a, b| (a[taller_idx]).cmp(
                      &(b[taller_idx])))
        .unwrap_or(&vec![0,0])[taller_idx];      

      // Create people stacks
      let people_stacks: Vec<Vec<Vec<i32>>> = vec![vec![]; most_taller_in_front.len()];

      // Sort people provided into a stack where grouped by num-taller and heights are sub-sorted descending
      people.sort_unstable_by(|a, b| (a[taller_idx], b[height_idx]).cmp(&(b[taller_idx], a[height_idx])));

      // Push people onto the appropriate people stacks
      while let Some(person) = people.pop() {
        people_stacks[person[taller_idx]].push(person);
      }

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
    }
}