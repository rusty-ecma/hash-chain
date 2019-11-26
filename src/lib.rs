
use std::{
    borrow::Borrow,
    collections::HashMap,
    hash::Hash,
    ops::{Index},
};

pub struct ChainMap<K, V> {
    pub(crate) maps: Vec<HashMap<K, V>>,
}

impl<K: Hash + Eq, V> ChainMap<K, V> {
    pub fn new(map: HashMap<K, V>) -> Self {
        Self { maps: vec![map] }
    }
    /// Inserts a key-value pair into the map.
    /// If the map did not have this key present, None is returned.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let map = self.maps.last_mut()?;
        map.insert(key, value)
    }
    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// `Hash` and `Eq` on the borrowed form *must* match those for
    /// the key type.
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V> 
    where 
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        for map in self.maps.iter().rev() {
            if let Some(v) = map.get(key) {
                return Some(v);
            }
        }
        None
    }
    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// `Hash` and `Eq` on the borrowed form *must* match those for
    /// the key type.
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where 
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let map = self.maps.last_mut()?;
        map.get_mut(key)
    }

    pub fn new_child(&mut self) {
        self.maps.push(HashMap::new());
    }

    pub fn new_child_with(&mut self, map: HashMap<K, V>) {
        self.maps.push(map);
    }
}

impl<K: Hash + Eq, V> Default for ChainMap<K, V> {
    fn default() -> Self {
        Self {
            maps: vec![HashMap::new()],
        }
    }
}


impl<K, Q: ?Sized, V> Index<&Q> for ChainMap<K, V>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash,
{
    type Output = V;

    /// Returns a reference to the value corresponding to the supplied key.
    ///
    /// # Panics
    ///
    /// Panics if the key is not present in the `HashMap`.
    #[inline]
    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("no entry found for key")
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::default::Default;

    #[test]
    fn initialization() {
        let mut test_map = HashMap::new();
        test_map.insert("test", 1);
        let chain_map = ChainMap::new(test_map);

        assert!(chain_map.maps.len() > 0);
        assert_eq!(chain_map.maps[0].get("test"), Some(&1));
    }

    #[test]
    fn initialization_default() {
        let chain_map: ChainMap<(), ()> = ChainMap::default();

        assert!(chain_map.maps.len() > 0);
        assert!(chain_map.maps[0].is_empty());
    }

    #[test]
    fn insert() {
        let mut chain_map = ChainMap::default();
        assert!(chain_map.insert("test", 1).is_none());

        assert_eq!(chain_map.maps[0].get("test"), Some(&1));
    }

    #[test]
    fn get() {
        let mut chain_map = ChainMap::default();
        chain_map.insert("test", 1);
        
        assert_eq!(chain_map.get(&"test"), Some(&1));
    }
    
    #[test]
    fn get_mut() {
        let mut chain_map = ChainMap::default();
        chain_map.insert("test", 1);

        let test_value = chain_map.get_mut(&"test");
        assert_eq!(test_value, Some(&mut 1));
        *test_value.unwrap() += 1;
        let changed = chain_map.get(&"test");
        assert_eq!(changed, Some(&2));
    }

    #[test]
    fn index() {
        let mut chain_map = ChainMap::default();
        chain_map.insert("test", 1);
        
        assert_eq!(chain_map[&"test"], 1);
    }

    #[test]
    fn new_child() {
        let mut chain_map = ChainMap::default();
        chain_map.insert("test", 1);
        chain_map.new_child();
        assert!(chain_map.maps.len() > 1);
    }

    #[test]
    fn scopes() {
        let mut chain_map = ChainMap::default();
        chain_map.insert("x", 0);
        chain_map.insert("y", 2);
        chain_map.new_child();
        chain_map.insert("x", 1);
        assert_eq!(chain_map.get("x"), Some(&1));
        assert_eq!(chain_map.get("y"), Some(&2));

    }
}
