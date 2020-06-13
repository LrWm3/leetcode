//
// Largest Divisible Subset
//
// au: wmarsman
// dt: 2020-06-13
//
// Usage
//   see help()
//
use log::*;
use simplelog::*;
use std::cmp;
use std::env;
use std::thread;

/**
 * Help
 */
fn help(args: Vec<String>) {
    println!(
      "
      Problem: Largest Divisible Subset

        2020-06-13
        url: https://leetcode.com/explore/featured/card/june-leetcoding-challenge/540/week-2-june-8th-june-14th/3359/

      Description
  
        Given a set of distinct positive integers, find the largest subset such that every pair (Si, Sj) of elements in this subset satisfies:

        Si % Sj = 0 or Sj % Si = 0.
        
        If there are multiple solutions, return any subset is fine.
              
      Example 1:
  
        Input: [1,2,3]
        Output: [1,2] (of course, [1,3] will also be ok)
  
      Example 2:
  
        Input: [1,2,4,8]
        Output: [1,2,4,8]

      Note:
  
        Input is a set of distinct positive integers.
  
      Usage
          Supply the distinct, postive integers space delimited.
          {} <arg1> <arg2> ... <argN>
  ",
      args[0]
    );
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        let input = vec![1, 2, 3];

        // Can be either of these answers
        let expected = vec![1, 2];
        let expected2 = vec![1, 3];

        let result = Solution::largest_divisible_subset(input);

        let matches_any_expected = result == expected || result == expected2;
        assert_eq!(true, matches_any_expected);
    }

    #[test]
    fn test_2() {
        let input = vec![1, 2, 4, 8];
        let expected = vec![1, 2, 4, 8];

        let result = Solution::largest_divisible_subset(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_3() {
        // This test was failing.
        let _ = SimpleLogger::init(LevelFilter::Info, Config::default());
        let input = vec![359, 376, 43, 315, 167, 216, 777, 625, 498, 442, 172, 324, 987, 400, 280, 367, 371, 24, 418, 208, 812, 488, 861, 646, 63, 804, 863, 853, 102, 174, 443, 901, 486, 126, 419, 701, 254, 550, 48, 214, 873, 386, 965, 504, 753, 336, 527, 522, 895, 339, 361, 755, 423, 558, 551, 276, 11, 724, 70, 823, 624, 555, 300, 42, 607, 554, 84, 508, 953, 649, 732, 338, 613, 236, 90, 762, 612, 194, 452, 972, 140, 747, 209, 690, 22, 220, 413, 91, 36, 998, 341, 77, 956, 246, 512, 464, 198, 547, 888, 476, 782, 977, 776, 896, 940, 321, 347, 264, 621, 10, 829, 383, 939, 825, 441, 326, 822, 754, 130, 379, 265, 945, 577, 491, 252, 273, 792, 168, 699, 866, 319, 704, 708, 148, 230, 521, 914, 988, 846, 88, 121, 600, 217, 499, 513, 427, 344, 3, 242, 947, 627, 325, 146, 469, 375, 12, 815, 46, 67, 193, 648, 963, 876, 78, 366, 531, 49, 532, 475, 875, 398, 69, 821, 454, 497, 170, 922, 872, 533, 736, 917, 951, 609, 461, 598, 571, 118, 798, 981, 835, 113, 530, 799, 995, 930, 682, 38, 405, 557, 787, 377, 810, 278, 874, 331, 199, 97, 215, 286, 13, 165, 473, 115, 816, 584, 707, 237, 568, 72, 166, 249, 805, 247, 746, 534, 408, 759, 739, 925, 855, 305, 210, 219, 470, 807, 936, 974, 417, 519, 288, 15, 64, 438, 581, 455, 250, 503, 496, 145, 256, 327, 255, 346, 251, 109, 650, 813, 679, 119, 619, 721, 406, 593, 489, 924, 964, 563, 897, 27, 769, 687, 608, 224, 462, 432, 39, 937, 384, 990, 45, 33, 154, 723, 152, 772, 795, 364, 283, 833, 395, 495, 164, 181, 232, 116, 899, 458, 548, 191, 320, 889, 587, 353, 661, 856, 814, 764, 529, 737, 948, 127, 335, 695, 960, 858, 801, 543, 916, 588, 478, 103, 592, 20, 481, 958, 618, 334, 424, 397, 694, 314, 158, 114, 700, 381, 287, 683, 966, 459, 923, 902, 332, 892, 235, 938, 178, 431, 631, 296, 885, 820, 409, 585, 141, 223, 535, 688, 258, 689, 884, 720, 365, 611, 277, 985, 684, 416, 666, 182, 961, 108, 355, 525, 862, 412, 549, 186, 244, 589, 421, 52, 76, 718, 352, 702, 510, 117, 290, 692, 603, 864, 323, 388, 536, 392, 151, 436, 350, 788, 75, 900, 490, 306, 975, 207, 261, 870, 188, 729, 231, 485, 348, 507, 676, 238, 111, 180, 984, 135, 771, 671, 51, 1, 997, 675, 869, 950, 445, 434, 92, 137, 221, 907, 245, 17, 794, 360, 935, 370, 239, 362, 175, 620, 973, 784, 106, 136, 122, 281, 426, 196, 134, 68, 634, 672, 28, 385, 411, 526, 735, 633, 841, 227, 86, 500, 653, 906, 933, 932, 129, 435, 756, 262, 698, 329, 204, 941, 614, 668, 139, 403, 229, 243, 808, 857, 659, 640, 545, 345, 82, 228, 516, 734, 566, 868, 414, 474, 506, 363, 87, 173, 578, 575, 312, 169, 908, 929, 444, 685, 657, 23, 524, 358, 225, 9, 41, 999, 834, 546, 920, 849, 456, 93, 651, 433, 586, 882, 942, 457, 62, 839, 818, 260, 369, 773, 890, 865, 596, 98, 271, 669, 962, 311, 996, 160, 200, 767, 539, 163, 800, 757, 582, 343, 538, 131, 567, 446, 213, 378, 959, 299, 915, 761, 313, 845, 712, 330, 253, 573, 18, 138, 317, 56, 691, 349, 605, 463, 652, 781, 992, 422, 32, 664, 711, 284, 741, 289, 57, 697, 368, 583, 943, 40, 298, 430, 851, 913, 745, 65, 179, 705, 630, 401, 674, 465, 487, 878, 477, 240, 35, 572, 838, 968, 678, 342, 775, 30, 806, 680, 969, 2, 241, 909, 803, 979, 460, 518, 156, 85, 643, 850, 597, 843, 89];

        let expected = vec![1,3,9,18,36,72,216,432,864];
        let result = Solution::largest_divisible_subset(input);

        assert_eq!(expected, result);
    }
}

struct Solution {}
impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![];
        }

        // Clone and sort input
        let mut vec = nums.clone();
        vec.sort();
        let vec_len = vec.len();

        // Lets use threads for fun.
        let threads_max: usize = 16;
        let threads_num: usize = cmp::min(threads_max, nums.len());
        let thread_v_len = (vec_len + vec_len / threads_num) / threads_num;
        debug!("thread_v_len={}", thread_v_len);

        let mut threads = vec![];

        for i in 0..threads_num {
            let thread_v_beg_i = i * thread_v_len;
            let thread_v_end_i = cmp::min(thread_v_beg_i + thread_v_len, vec_len);

            if thread_v_beg_i != thread_v_end_i {
                debug!(
                    "thread {} start - [{},{}]",
                    i, thread_v_beg_i, thread_v_end_i
                );
                let thread_v_inds = vec[thread_v_beg_i..thread_v_end_i].to_vec();

                // Need to copy here to pass across thread boundaries.
                // Surely there is a better way?
                let thread_v = vec.to_vec();

                // Start our thread.
                threads.push(thread::spawn(move || -> Vec<Vec<i32>> {
                    let mut thread_v_sols: Vec<Vec<i32>> = vec![];
                    for thread_v_ind in thread_v_inds {
                        thread_v_sols
                            .push(Solution::all_match_subset_for_value(thread_v_ind, &thread_v));
                    }
                    thread_v_sols
                }));
            }
        }

        // Collect each thread's intermediate results into a new Vec.
        let mut num_pairs_all: Vec<Vec<i32>> = vec![];
        for thread in threads {
            // collect each child thread's return-value
            let mut num_pairs_thread = thread.join().unwrap();
            num_pairs_all.append(&mut num_pairs_thread);
        }

        // Log the results for each i.
        for (i, num_pair) in num_pairs_all.iter().enumerate() {
            debug!("num_pair[{}]={:?}", i, num_pair);
        }

        // Return the result with the largest number of pairs.
        return num_pairs_all
            .iter()
            .max_by_key(|num_pair| num_pair.len())
            .unwrap()
            .to_vec();
    }

    // Finds and returns all match subset for a given value ni
    fn all_match_subset_for_value(ni: i32, v: &Vec<i32>) -> Vec<i32> {
        let mut ni_pairs = vec![];
        for nj in v.iter() {
            if Solution::condition(ni, *nj) {
                debug!("match(pot): ni={}, nj={}", ni, nj);
                if ni_pairs
                    .iter()
                    .all(|ni_pair_val| Solution::condition(*ni_pair_val, *nj))
                {
                    debug!("match(act): ni={}, nj={}", ni, nj);
                    ni_pairs.push(*nj);
                }
            }
        }
        ni_pairs
    }

    // Interface to obscure underlying comparison from implementation
    fn condition(ni: i32, nj: i32) -> bool {
        Solution::is_either_mod_pair(ni, nj)
    }

    // Assembles the condition and arguments.
    fn is_either_mod_pair(ni: i32, nj: i32) -> bool {
        Solution::either_true(ni, nj, Solution::is_mod_pair)
    }

    // Represents how the supplied condition should be applied; in this case
    fn either_true<F>(ni: i32, nj: i32, f: F) -> bool
    where
        F: Fn(i32, i32) -> bool,
    {
        return f(ni, nj) || f(nj, ni);
    }

    // Represents the condition we're testing for between values.
    fn is_mod_pair(ni: i32, nj: i32) -> bool {
        ni % nj == 0
    }
}

fn main() -> Result<(), ()> {
    // Set-up global stdout/stderr logger with log-level as debug
    let _ = SimpleLogger::init(LevelFilter::Debug, Config::default());

    let args: Vec<String> = env::args().collect();
    match args.len() {
        // no arguments passed
        1 => {
            help(args);
            println!("No arguments provided!");
            return Result::Err(());
        }
        // one argument passed
        _ => {
            let input: Vec<i32> = args[1..args.len()]
                .iter()
                .map(|arg| {
                    arg.parse::<i32>()
                        .expect("Could not unwrap one of the provided arguments!")
                })
                .collect();
            println!("{:?}", Solution::largest_divisible_subset(input));
            return Result::Ok(());
        }
    }
}
