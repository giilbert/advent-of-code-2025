use std::collections::{HashMap, HashSet};

// use i256::u256;

use crate::harness::Harness;

pub fn part_1(harness: &Harness) -> String {
    let start_index = harness
        .input()
        .lines()
        .next()
        .expect("expected at least one line")
        .chars()
        .position(|c| c == 'S')
        .expect("expected to find 'S' in the first line");

    let mut total_splits = 0;
    let mut current_line_beams = HashSet::new();
    current_line_beams.insert(start_index);

    for line in harness.input().lines().skip(1) {
        let mut next_line_beams: HashSet<usize> = HashSet::new();

        for &beam_position in &current_line_beams {
            if line
                .chars()
                .nth(beam_position)
                .expect("beam_position out of bound")
                == '^'
            {
                next_line_beams.insert(beam_position - 1);
                next_line_beams.insert(beam_position + 1);
                total_splits += 1;
            } else {
                next_line_beams.insert(beam_position);
            }
        }

        current_line_beams = next_line_beams;
    }

    total_splits.to_string()
}

// #[repr(transparent)]
// #[derive(Clone, Copy, PartialEq, Eq, Hash)]
// struct EncodedPath {
//     bmp: u256,
// }

// const ENCODE_LEFT: u8 = 0b01;
// const ENCODE_RIGHT: u8 = 0b10;
// const ENCODE_STRAIGHT: u8 = 0b11;

// impl EncodedPath {
//     #[inline(always)]
//     fn new() -> Self {
//         EncodedPath {
//             bmp: u256::from_u8(0),
//         }
//     }

//     #[inline(always)]
//     fn left(&self) -> EncodedPath {
//         EncodedPath {
//             bmp: (self.bmp << 2) | u256::from_u8(ENCODE_LEFT),
//         }
//     }

//     #[inline(always)]
//     fn right(&self) -> EncodedPath {
//         EncodedPath {
//             bmp: (self.bmp << 2) | u256::from_u8(ENCODE_RIGHT),
//         }
//     }

//     #[inline(always)]
//     fn straight(&self) -> EncodedPath {
//         EncodedPath {
//             bmp: (self.bmp << 2) | u256::from_u8(ENCODE_STRAIGHT),
//         }
//     }
// }

// impl Debug for EncodedPath {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for i in (0..128).rev() {
//             let two_bits: u256 = (self.bmp >> (i * 2)) & u256::from_u8(0b11);
//             let c = match two_bits.as_u8() {
//                 ENCODE_LEFT => 'L',
//                 ENCODE_RIGHT => 'R',
//                 ENCODE_STRAIGHT => 'S',
//                 _ => '?',
//             };
//             write!(f, "{}", c)?;
//         }

//         Ok(())
//     }
// }

pub fn part_2(harness: &Harness) -> String {
    let start_index = harness
        .input()
        .lines()
        .next()
        .expect("expected at least one line")
        .chars()
        .position(|c| c == 'S')
        .expect("expected to find 'S' in the first line");

    // maps the current x-index of the beam to the number of paths that have reached there
    let mut current_line_beams = HashMap::<usize, u64>::new();
    current_line_beams.insert(start_index, 1);

    for line in harness.input().lines().skip(1) {
        let mut next_line_beams: HashMap<usize, u64> = HashMap::new();

        for (&beam_position, &beam_count) in &current_line_beams {
            if line
                .chars()
                .nth(beam_position)
                .expect("beam_position out of bound")
                == '^'
            {
                next_line_beams
                    .entry(beam_position - 1)
                    .and_modify(|c| *c += beam_count)
                    .or_insert(beam_count);
                next_line_beams
                    .entry(beam_position + 1)
                    .and_modify(|c| *c += beam_count)
                    .or_insert(beam_count);
            } else {
                next_line_beams
                    .entry(beam_position)
                    .and_modify(|c| *c += beam_count)
                    .or_insert(beam_count);
            }
        }

        println!("After line '{}': {:?}", line, next_line_beams);
        current_line_beams = next_line_beams;
    }

    current_line_beams.values().sum::<u64>().to_string()
}
