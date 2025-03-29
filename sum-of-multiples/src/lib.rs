use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut integers = HashSet::new();

    for i in factors.iter().filter(|x| **x != 0) {
        let mut idx = 1_u32;
        while idx < limit {
            if idx % i == 0 {
                integers.insert(idx);
            }
            idx += 1;
        }
    }

    integers.iter().sum()
}
