use dashmap::{DashMap, DashSet};
use rand::{random, Rng};
use std::collections::{HashSet, BinaryHeap};
use crate::pool::room::Room;

pub mod room;

pub struct Pool<'a> {
    levels: DashMap<i32, BinaryHeap<&'a Room>>,

}

impl Pool {
    pub fn new() -> Pool {
        Pool {
            rooms: vec![],
            pool: DashMap::new(),
        }
    }

    pub fn get(&self, n: usize) -> Vec<&String> {
        let mut rng = rand::thread_rng();
        let mut s = HashSet::new();
        let mut ret = vec![];
        while n > 0 {
            rng.gen_range(0, self.rooms.len())
            if let Some(x) = self.rooms.get() {
                ret.push(x);
            }
        }
        ret
    }


}
