use std::collections::HashSet;
#[derive(Clone)]
pub struct ChainSet<T> {
    leaf: HashSet<T>,
    sets: Vec<HashSet<T>>,
}
impl<T> std::fmt::Display for ChainSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter)-> std::fmt::Result {
        write!(f, "ChainSet")
    }
}
impl<T> std::fmt::Debug for ChainSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter)-> std::fmt::Result {
        write!(f, "ChainSet")
    }
}

impl<T> ChainSet<T> 
where T: std::cmp::Eq + std::hash::Hash {
    pub fn new() -> Self {
        Self {
            leaf: HashSet::new(),
            sets: vec![]
        }
    }
    pub fn insert(&mut self, key: T) -> bool {
        if !self.contains(&key) {
            self.leaf.insert(key);
            return true
        } else {
            false
        }
    }
    pub fn new_child(&mut self) {
        let old_leaf = std::mem::replace(&mut self.leaf, HashSet::new());
        self.sets.push(old_leaf);
    }
    pub fn pop_child(&mut self) -> Option<HashSet<T>> {
        let ret = std::mem::replace(&mut self.leaf, self.sets.pop()?);
        Some(ret)
    }
    pub fn contains<Q: ?Sized>(&self, key: &Q) -> bool
        where T: std::borrow::Borrow<Q>,
              Q: std::hash::Hash + Eq
    {
        if self.leaf.contains(key){
            return true;
        }
        for set in self.sets.iter() {
            if set.contains(key){
                return true;
            }
        }
        false
    }
}