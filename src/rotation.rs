use std::collections::HashSet;

use nannou::prelude::Pow;

type AxisId = usize;

struct AxisTransform {
    axis_index: usize,
    negated: bool,
}

struct AxisPermutation {
    items: Vec<AxisId>,
    parity: bool,
}

pub fn permutations(num_axes: usize) -> Vec<AxisPermutation> {
    let num_arrangements = num_axes.pow(num_axes as u32);
    (0..num_arrangements)
        .map(|i| {
            (0..num_axes)
                .map(|a| (i / num_axes.pow(a as u32)) % num_axes)
                .collect::<Vec<_>>()
        })
        .filter(|arrangement| is_permutation(arrangement))
        .map(move |arrangement| AxisPermutation {
            items: arrangement,
            parity: parity(&arrangement),
        })
        .collect()
}

/// calculate parity of permutation by counting cycle parities
pub fn parity(arr: &[AxisId]) -> bool {
    let mut parity = false;
    let mut visited = vec![false; arr.len()];
    loop {
        // first non-visited node
        match visited.iter().position(|&x| !x) {
            Some(first) => {
                let mut idx = first;

                let mut cycle_count = 0;
                // mark first in cycle as visited
                while !visited[idx] {
                    visited[idx] = true;
                    idx = arr[idx];
                    cycle_count += 1;
                }
                // finshed a cycle, factor in parity
                parity ^= (cycle_count - 1) % 2 == 1;
            }
            // have visited every one
            None => break,
        }
    }
    parity
}

fn permutations_with_parity(len: usize) -> Vec<AxisPermutation> {
    todo!();
}
