use std::{borrow::Borrow, collections::HashSet, hash::Hash, mem::replace};

pub struct ChainSet<T> {
    pub(crate) sets: Vec<HashSet<T>>,
}

impl<T: Hash + Eq> ChainSet<T> {
    pub fn new(set: HashSet<T>) -> Self {
        Self { sets: vec![set] }
    }

    pub fn insert(&mut self, value: T) -> bool {
        if let Some(set) = self.sets.last_mut() {
            set.insert(value)
        } else {
            let mut set = HashSet::new();
            set.insert(value);
            self.sets.push(set);
            false
        }
    }

    pub fn get<Q: ?Sized>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        for set in self.sets.iter().rev() {
            if let Some(v) = set.get(value) {
                return Some(v);
            }
        }
        None
    }

    pub fn new_child(&mut self) {
        self.sets.push(HashSet::new());
    }

    pub fn new_child_with(&mut self, map: HashSet<T>) {
        self.sets.push(map);
    }

    pub fn remove_child(&mut self) -> Option<HashSet<T>> {
        if self.sets.len() == 1 {
            let ret = replace(&mut self.sets[0], HashSet::new());
            Some(ret)
        } else {
            self.sets.pop()
        }
    }
}

impl<T: Hash + Eq> Default for ChainSet<T> {
    fn default() -> Self {
        Self {
            sets: vec![HashSet::new()],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::default::Default;

    #[test]
    fn initialization() {
        let mut test_set = HashSet::new();
        test_set.insert("test");
        let chain_set = ChainSet::new(test_set);

        assert!(chain_set.sets.len() > 0);
        assert_eq!(chain_set.sets[0].get("test"), Some(&"test"));
    }

    #[test]
    fn initialization_default() {
        let chain_set: ChainSet<()> = ChainSet::default();

        assert!(chain_set.sets.len() > 0);
        assert!(chain_set.sets[0].is_empty());
    }

    #[test]
    fn insert() {
        let mut chain_set = ChainSet::default();
        assert!(chain_set.insert("test"));

        assert_eq!(chain_set.sets[0].get("test"), Some(&"test"));
    }

    #[test]
    fn get() {
        let mut chain_set = ChainSet::default();
        chain_set.insert("test");

        assert_eq!(chain_set.get(&"test"), Some(&"test"));
    }

    #[test]
    fn get_none() {
        let chain_set: ChainSet<&str> = ChainSet::default();
        assert_eq!(chain_set.get(&"test"), None);
    }

    #[test]
    fn new_child() {
        let mut chain_set = ChainSet::default();
        chain_set.insert("test");
        chain_set.new_child();
        assert!(chain_set.sets.len() > 1);
    }

    #[test]
    #[ignore]
    fn scopes() {
        let mut chain_set = ChainSet::default();
        chain_set.insert("x");
        chain_set.insert("y");
        chain_set.new_child();
        assert!(chain_set.insert("x"));
    }

    #[test]
    fn remove_child() {
        let mut chain_set = ChainSet::default();
        chain_set.insert("x");
        chain_set.insert("y");
        chain_set.new_child();
        chain_set.insert("x");
        let ret = chain_set.remove_child().unwrap();
        assert_eq!(ret.get("x"), Some(&"x"));
        assert_eq!(chain_set.get("x"), Some(&"x"));
    }

    #[test]
    fn remove_child_length_1() {
        let mut chain_set = ChainSet::default();
        chain_set.insert("x");
        let _ = chain_set.remove_child();
        assert_eq!(chain_set.get("x"), None);
        assert!(chain_set.sets.len() == 1);
    }
}
