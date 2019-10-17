#[macro_use]
extern crate lazy_static;

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use rstar::RTree;
use std::collections::HashMap;
use std::sync::RwLock;

lazy_static! {
  static ref TREES: RwLock<HashMap<String, RTree<[f64; 2]>>> = RwLock::new(HashMap::new());
}

#[wasm_bindgen]
pub fn create_tree(uid: String) {
  let mut trees = TREES.write().unwrap();
  trees.insert(uid, RTree::new());
}

#[wasm_bindgen]
pub fn insert_to_tree(uid: String, point: &[f64]) {
  let mut trees = TREES.write().unwrap();
  trees.get_mut(&uid).unwrap().insert([point[0], point[1]]);
}

#[wasm_bindgen]
pub fn remove_from_tree(uid: String) {
  let mut trees = TREES.write().unwrap();
  trees.remove(&uid);
}

#[wasm_bindgen]
pub fn nearest_in_tree(uid: String, point: &[f64]) -> Box<[f64]> {
  let trees = TREES.read().unwrap();
  let point_exist = trees.get(&uid).unwrap().nearest_neighbor(&[point[0], point[1]]);
  let mut resp = vec![];
  if point_exist.is_some() {
    let point = point_exist.unwrap();
    resp.push(point[0]);
    resp.push(point[1]);
  }
  return resp.into_boxed_slice()
}

pub fn main() {}
