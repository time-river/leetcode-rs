// https://leetcode.com/problems/jump-game-iv/

use std::collections::HashMap;
use std::collections::HashSet;

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut graph: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut reached: HashSet<i32> = HashSet::new();
        let mut ans: i32 = 0;

        for (i, key) in arr.iter().enumerate() {
            match graph.get_mut(key) {
                None => {
                    graph.insert(*key, vec![i]);
                }
                Some(val) => {
                    val.push(i);
                }
            }
        }

        let mut curs: HashSet<i32> = [0].iter().cloned().collect();
        while !curs.is_empty() {
            let mut nex: HashSet<i32> = HashSet::new();

            for node in curs.iter() {
                if *node as usize == arr.len()-1 {
                    return ans;
                }

                // check the same value
                for &idx in graph[&arr[*node as usize]].iter() {
                    let key = idx as i32;
                    if !reached.contains(&key) {
                        nex.insert(key);
                        reached.insert(key);
                    }
                }

                // clear the list to prevent redundant search, otherwise timeout
                graph.get_mut(&arr[*node as usize]).unwrap().clear();

                // check the neighbors
                for &neighbor in [*node-1, *node+1].iter() {
                    if neighbor >= 0 && (neighbor as usize) < arr.len()
                        && !reached.contains(&neighbor) {
                            nex.insert(neighbor);
                            reached.insert(neighbor);
                    }
                }
            }

            curs = nex;
            ans += 1;
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::min_jumps(vec![100,-23,-23,404,100,23,23,23,3,404]), 3);
        assert_eq!(Solution::min_jumps(vec![6,1,9]), 2);
    }
}
