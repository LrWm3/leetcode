/**
 * Implementation
 */
struct RandomizedSet {
    objToInd: HashMap<i32, usize>,
    indToObj: HashMap<usize, i32>,
    size: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&self, val: i32) -> bool {}

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&self, val: i32) -> bool {}

    /** Get a random element from the set. */
    fn get_random(&self) -> i32 {}
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

fn main() {
    println!("Hello, world!");
}
