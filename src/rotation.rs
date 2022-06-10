use std::collections::HashSet;

type AxisId = usize;

#[derive(Debug)]
pub struct TransformedAxis {
    input_axis: usize,
    negated: bool,
}

#[derive(Debug)]
pub struct AxisPermutation {
    items: Vec<AxisId>,
    parity: bool,
}

type RotationConfiguration = Vec<TransformedAxis>;

/// https://math.stackexchange.com/questions/2603222/simple-rotations-in-n-dimensions-limited-to-right-angle-rotations
pub fn rotation_permutations(num_axes: usize) -> Vec<RotationConfiguration> {
    let num_arrangements = num_axes.pow(num_axes as u32);
    (0..num_arrangements)
        .map(|i| {
            (0..num_axes)
                .map(|a| (i / num_axes.pow(a as u32)) % num_axes)
                .collect::<Vec<_>>()
        })
        .filter(|arrangement| is_permutation(arrangement))
        .map(move |arrangement| {
            let parity = parity(&arrangement);
            AxisPermutation {
                items: arrangement,
                parity,
            }
        })
        .flat_map(|permutation| enumerate_negations(permutation))
        .collect()
}

fn is_permutation(arr: &[AxisId]) -> bool {
    let mut unique = HashSet::new();
    arr.iter().all(move |x| unique.insert(x))
}

/// calculate parity of permutation by counting cycle parities
/// https://stackoverflow.com/questions/20702782/efficiently-determine-the-parity-of-a-permutation
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

/// Given a permutation, give a list of all the different ways the axes can be negated. The parity
/// of the number of negations must equal that of the parity of the permutation.
fn enumerate_negations(permutation: AxisPermutation) -> Vec<RotationConfiguration> {
    let len = permutation.items.len() as u32;
    let mut out = Vec::new();
    // there are 2^(n-1) possibilities for sign flips. The last item's negation is determined by
    // the parity of the permutation. A sign flip is a binary operation (flip or not flip), so this
    // can be encoded by an integer that counts up to enumerate each possibility:
    // `negation_configuration_int`.
    for negation_configuration_int in 0..2u32.pow(len - 1) {
        let mut negation_configuration = permutation
            .items
            .iter()
            .map(|axis| TransformedAxis {
                input_axis: *axis,
                negated: false,
            })
            .collect::<Vec<_>>();

        // for each bit, if it's set in negation_configuration_int, then set the corresponding axis
        // to be negated.
        for bit_idx in 0..(len - 1) {
            if bit(negation_configuration_int, bit_idx) {
                negation_configuration[bit_idx as usize].negated = true;
            }
        }

        // Parity of number of negations must equal parity of the permutation. Fiddle the last
        // axis's negation to make this true.
        let negation_parity = negation_configuration_int.count_ones() % 2 == 1;
        // binary XOR => different parity, so negate the axis
        negation_configuration[(len - 1) as usize].negated = permutation.parity ^ negation_parity;

        out.push(negation_configuration);
    }
    out
}

fn bit(n: u32, index: u32) -> bool {
    n & (1 << index) != 0
}
