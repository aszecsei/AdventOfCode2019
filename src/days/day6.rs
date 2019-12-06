use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use id_arena::{Arena, Id};
use std::collections::HashMap;

// ======================================================
// DAY 6
// ======================================================

#[aoc_generator(day6)]
pub fn input_generator_day6(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|x| {
            let orbit = x
                .split(')')
                .map(|id| id.trim().to_lowercase())
                .collect_vec();
            (orbit[0].to_owned(), orbit[1].to_owned())
        })
        .collect_vec()
}

type OrbitId = Id<Orbit>;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Orbit {
    pub id: OrbitId,
    pub text_id: String,
    pub orbits: Option<OrbitId>,

    dist_you: isize,
}

impl Orbit {
    pub fn new(id: OrbitId, text_id: String) -> Self {
        Orbit {
            id,
            text_id,
            orbits: None,

            dist_you: -1,
        }
    }

    pub fn orbit_count(&self, arena: &Arena<Orbit>) -> usize {
        if let Some(pa) = self.orbits {
            let pa = &arena[pa];
            1 + pa.orbit_count(arena)
        } else {
            0
        }
    }
}

#[aoc(day6, part1)]
pub fn solve_day6_part1(input: &[(String, String)]) -> usize {
    let ids = {
        let id1_iter = input.iter().map(|pair| &pair.0);
        let id2_iter = input.iter().map(|pair| &pair.1);
        id1_iter.chain(id2_iter).unique().collect_vec()
    };
    let mut arena = Arena::<Orbit>::new();

    let mut orbits: HashMap<String, OrbitId> = HashMap::default();
    for &id in ids.iter() {
        let ob = arena.alloc_with_id(|arena_id| Orbit::new(arena_id, id.to_owned()));
        orbits.insert(id.to_owned(), ob);
    }

    for orbit in input.iter() {
        let pa_id = orbits[&orbit.0];
        let ob_id = orbits[&orbit.1];
        let mut ob = arena.get_mut(ob_id).unwrap();
        ob.orbits = Some(pa_id);
    }

    ids.iter()
        .map(|id| {
            arena
                .get(orbits[id.to_owned()])
                .unwrap()
                .orbit_count(&arena)
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_day6_part2(input: &[(String, String)]) -> usize {
    let ids = {
        let id1_iter = input.iter().map(|pair| &pair.0);
        let id2_iter = input.iter().map(|pair| &pair.1);
        id1_iter.chain(id2_iter).unique().collect_vec()
    };
    let mut arena = Arena::<Orbit>::new();

    let mut orbits: HashMap<String, OrbitId> = HashMap::default();
    for &id in ids.iter() {
        let ob = arena.alloc_with_id(|arena_id| Orbit::new(arena_id, id.to_owned()));
        orbits.insert(id.to_owned(), ob);
    }

    for orbit in input.iter() {
        let pa_id = orbits[&orbit.0];
        let ob_id = orbits[&orbit.1];
        let mut ob = arena.get_mut(ob_id).unwrap();
        ob.orbits = Some(pa_id);
    }

    // Trace up from "you"
    {
        let mut counter = 0;
        let node_id = arena.get(orbits["you"]).unwrap().orbits.unwrap();
        let mut node = arena.get_mut(node_id).unwrap();
        loop {
            node.dist_you = counter;
            if let Some(p_id) = node.orbits {
                node = arena.get_mut(p_id).unwrap();
                counter += 1;
            } else {
                break;
            }
        }
    }
    // Trace up from "san"
    {
        let mut counter = 0;
        let node_id = arena.get(orbits["san"]).unwrap().orbits.unwrap();
        let mut node = arena.get_mut(node_id).unwrap();
        loop {
            // Check if we've already reached "you"
            if node.dist_you >= 0 {
                return counter + node.dist_you as usize;
            }
            if let Some(p_id) = node.orbits {
                node = arena.get_mut(p_id).unwrap();
                counter += 1;
            } else {
                break;
            }
        }
    }

    panic!("Something went wrong!");
}
