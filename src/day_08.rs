use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
};

use itertools::Itertools;

use crate::{harness::Harness, next_tuple};

#[derive(Debug, PartialEq, Eq, Hash)]
struct JunctionBox {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl JunctionBox {
    pub fn distance_to(&self, other: &JunctionBox) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

impl std::fmt::Display for JunctionBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PairWithDistance<'a> {
    pub box_a: &'a JunctionBox,
    pub box_b: &'a JunctionBox,
    pub distance: i64,
}

impl std::cmp::PartialOrd for PairWithDistance<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.distance.cmp(&other.distance))
    }
}

impl std::cmp::Ord for PairWithDistance<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

pub fn part_1(harness: &Harness) -> String {
    let all_junction_boxes = harness
        .input()
        .lines()
        .map(|line| {
            let mut split = line.split(",");
            let (x, y, z) = next_tuple!(split, i64, i64, i64);
            JunctionBox { x, y, z }
        })
        .collect::<Vec<JunctionBox>>();

    let mut top_k: BinaryHeap<PairWithDistance> = BinaryHeap::new();

    for (i, box_a) in all_junction_boxes.iter().enumerate() {
        for box_b in all_junction_boxes.iter().skip(i + 1) {
            let distance = box_a.distance_to(box_b);
            let pair = PairWithDistance {
                box_a,
                box_b,
                distance,
            };
            top_k.push(pair);
        }
    }

    let mut circuits: Vec<RefCell<Vec<&JunctionBox>>> = Vec::new();

    fn update_circuit<'a>(
        circuits: &mut Vec<RefCell<Vec<&'a JunctionBox>>>,
        box_a: &'a JunctionBox,
        box_b: &'a JunctionBox,
    ) {
        let is_box_a_in_circuit = circuits.iter().any(|c| c.borrow().contains(&box_a));
        let is_box_b_in_circuit = circuits.iter().any(|c| c.borrow().contains(&box_b));

        println!("update_circuit({box_a}, {box_b})");
        for circuit in &*circuits {
            // println!("- {circuit:?}");
            println!(
                "- {}",
                circuit.borrow().iter().map(|c| format!("{}", c)).join(" ")
            );
        }

        match (is_box_a_in_circuit, is_box_b_in_circuit) {
            // both boxes are in a circuit, move circuit b to a
            (true, true) => {
                let (box_a_circuit_index, box_a_circuit) = circuits
                    .iter()
                    .enumerate()
                    .find(|(_index, c)| c.borrow().contains(&box_a))
                    .expect("circuit not found");
                let (box_b_circuit_index, box_b_circuit) = circuits
                    .iter()
                    .enumerate()
                    .find(|(_index, c)| c.borrow().contains(&box_b))
                    .expect("circuit not found");

                // println!(
                //     "box a found in {box_a_circuit_index}, box b found in {box_b_circuit_index}"
                // );

                if box_a_circuit_index == box_b_circuit_index {
                    println!("--> link already exists. do nothing\n");
                    return;
                }

                println!("--> connecting b to a");

                // add the boxes in b to a
                for box_in_b in box_b_circuit.borrow().iter() {
                    box_a_circuit.borrow_mut().push(&box_in_b);
                }

                // remove circuit b
                circuits.remove(box_b_circuit_index);
            }
            // box a is in a circuit, box b isnt: add box b to a
            (true, false) => {
                println!("--> add box b to a");
                let box_a_circuit = circuits
                    .iter_mut()
                    .find(|c| c.borrow().contains(&box_a))
                    .expect("circuit not found");
                box_a_circuit.borrow_mut().push(box_b);
            }
            // box b is in a circuit, box a isnt: add box a to b
            (false, true) => {
                println!("--> add box a to b");
                let box_b_circuit = circuits
                    .iter_mut()
                    .find(|c| c.borrow().contains(&box_b))
                    .expect("circuit not found");
                box_b_circuit.borrow_mut().push(box_a);
            }
            // none of the boxes are in a circuit, create a circuit with both boxes
            (false, false) => {
                println!("--> start new circuit");
                circuits.push(RefCell::new(vec![box_a, box_b]));
            }
        }

        println!("");
    }

    let mut top_k_list = top_k.iter().collect::<Vec<_>>();
    top_k_list.sort_by(|left, right| left.distance.cmp(&right.distance));

    for pair in top_k_list {
        println!("- dist={}", pair.distance);
        update_circuit(&mut circuits, pair.box_a, pair.box_b);
    }

    circuits.sort_by(|a, b| b.borrow().len().cmp(&a.borrow().len()));

    (circuits[0].borrow().len() * circuits[1].borrow().len() * circuits[2].borrow().len())
        .to_string()
}

pub fn part_2(harness: &Harness) -> String {
    let all_junction_boxes = harness
        .input()
        .lines()
        .map(|line| {
            let mut split = line.split(",");
            let (x, y, z) = next_tuple!(split, i64, i64, i64);
            JunctionBox { x, y, z }
        })
        .collect::<Vec<JunctionBox>>();

    let mut top_k: BinaryHeap<Reverse<PairWithDistance>> = BinaryHeap::new();

    for (i, box_a) in all_junction_boxes.iter().enumerate() {
        for box_b in all_junction_boxes.iter().skip(i + 1) {
            let distance = box_a.distance_to(box_b);
            let pair = PairWithDistance {
                box_a,
                box_b,
                distance,
            };
            top_k.push(Reverse(pair));
        }
    }

    let mut in_circuit = HashSet::new();
    while let Some(Reverse(closest_pair)) = top_k.pop() {
        println!("closest pair: {:?}", closest_pair);
        in_circuit.insert(closest_pair.box_a);
        in_circuit.insert(closest_pair.box_b);

        if in_circuit.len() == all_junction_boxes.len() {
            return (closest_pair.box_a.x * closest_pair.box_b.x).to_string();
        }
    }

    unreachable!()
}
