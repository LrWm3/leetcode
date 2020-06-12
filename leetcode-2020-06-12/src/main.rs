use rand::Rng;
use std::collections::HashMap;

/**
 * Implementation
 */
struct RandomizedSet {
    objToInd: HashMap<i32, usize>,
    objs: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        return RandomizedSet {
            objToInd: HashMap::new(),
            objs: vec![],
        };
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        if !self.objToInd.contains_key(&val) {
            let size = self.objs.len();
            self.objToInd.insert(val, size);
            self.objs.push(val);
            return true;
        } else {
            return false;
        }
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        if self.objToInd.contains_key(&val) {
            // Removal has the following happen:
            // 1. First, we get the index of the object
            // 2. If the object is not at the end of the list;
            //   2a. Next, we retrieve the object at the end of the list
            //   2b. Next, we swap the object at the end of the list with object at index
            //   2c. Then, we remove the object at the end of the list
            //   2d. Next, we update the object in the map at key to instead be the object at the end of the list
            // 3. If the object is at the end of the list;
            //   3a. Pop the object.
            // 4. Next, we remove the object associated with key at the last index

            // step 1
            let ind = self.objToInd.get(&val).unwrap().clone();
            // step 2
            if ind != self.objs.len() - 1 {
                self.objs.swap_remove(ind);
                self.objToInd.insert(self.objs[ind].clone(), ind);
            } else {
                // step 3
                self.objs.pop();
            }
            // step 4
            self.objToInd.remove(&val);

            return true;
        } else {
            return false;
        }
    }

    /** Get a random element from the set. */
    fn get_random(&mut self) -> i32 {
        let rand_ind: usize = rand::thread_rng().gen_range(0, self.objs.len());
        return self.objs[rand_ind].clone();
    }
}
