use std::{
    borrow::Borrow,
    collections::{hash_map::RandomState},
    hash::{BuildHasher, Hash},
    mem::take,
    ops::Index,
};
use im_rc::{Vector, HashMap};

#[derive(Clone)]
pub struct LockedChainMap<K, V, S = RandomState> {
    pub(crate) maps: Vector<HashMap<K, V, S>>,
}

impl<K, V, S: BuildHasher> LockedChainMap<K, V, S>
where
    K: Hash + Eq + Clone,
    V: Clone,
    S: BuildHasher + Clone,
{
    pub fn new(map: HashMap<K, V, S>) -> Self {
        Self { maps: Vector::unit(map) }
    }
    /// Inserts a key-value pair into the map.
    /// If the map did not have this key present, None is returned.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let map = self.maps.get_mut(self.maps.len() - 1)?;
        map.insert(key, value)
    }

    pub fn insert_at(&mut self, idx: usize, key: K, value: V) -> Result<Option<V>, crate::Error> {
        if let Some(map) = self.maps.get_mut(idx) {
            Ok(map.insert(key, value))
        } else {
            Err(crate::Error::IndexOutOfRange)
        }
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
        for map in self.maps.iter_mut().rev() {
            if let Some(v) = map.get_mut(key) {
                return Some(v);
            }
        }
        None
    }

    pub fn get_before<Q: ?Sized>(&self, idx: usize, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        for i in (0..idx).rev() {
            if let Some(map) = self.maps.get(i) {
                if let Some(v) = map.get(key) {
                    return Some(v)
                }
            }
        }
        None
    }

    pub fn get_before_mut<Q: ?Sized>(&mut self, idx: usize, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        for (i, map) in self.maps.iter_mut().enumerate().rev() {
            if i > idx {
                continue;
            } 
            if let Some(v) = map.get_mut(key) {
                return Some(v)
            }
        }
        None
    }

    pub fn new_child_with(&mut self, map: HashMap<K, V, S>) {
        self.maps.push_back(map);
    }

    pub fn last_has<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.has_at(self.maps.len() - 1, key)
    }

    pub fn has_at<Q: ?Sized>(&self, idx: usize, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        if let Some(map) = self.maps.get(idx) {
            map.contains_key(key)
        } else {
            false
        }
    }

    pub fn child_len(&self) -> usize {
        self.maps.len()
    }

    pub fn get_last_index<Q: ?Sized>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        for (i, map) in self.maps.iter().enumerate().rev() {
            if map.contains_key(key) {
                return Some(i);
            }
        }
        None
    }

    pub fn split_off(&mut self, idx: usize) -> Self {
        let maps = self.maps.split_off(idx);
        Self {
            maps,
        }
    }

    pub fn append(&mut self, other: Self) {
        self.maps.append(other.maps);
    }
}

impl<K: Hash + Eq + Clone, V: Clone, S: BuildHasher + Default + Clone> LockedChainMap<K, V, S> {
    pub fn new_child(&mut self) {
        self.maps.push_back(HashMap::default());
    }

    pub fn remove_child(&mut self) -> Option<HashMap<K, V, S>> {
        if self.maps.len() == 1 {
            let ret = take(&mut self.maps[0]);
            Some(ret)
        } else {
            self.maps.pop_back()
        }
    }
}

impl<K, V> Default for LockedChainMap<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone
{
    fn default() -> Self {
        let maps = Vector::unit(HashMap::new());
        Self {
            maps,
        }
    }
}

impl<K, Q: ?Sized, V: Clone, S> Index<&Q> for LockedChainMap<K, V, S>
where
    K: Eq + Hash + Borrow<Q> + Clone,
    Q: Eq + Hash + Clone,
    S: BuildHasher + Clone,
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

impl<K, V, S> PartialEq for LockedChainMap<K, V, S>
where
    K: Eq + Hash + Clone,
    V: PartialEq + Clone,
    S: std::hash::BuildHasher + Clone,
{
    fn eq(&self, other: &LockedChainMap<K, V, S>) -> bool {
        self.maps == other.maps
    }
}

impl<K, V, S> Eq for LockedChainMap<K, V, S>
where
    K: Eq + Hash + Clone,
    V: Eq + Clone,
    S: BuildHasher + Clone,
{
}

impl<K, V, S> core::fmt::Debug for LockedChainMap<K, V, S>
where
    K: Eq + Hash + core::fmt::Debug + Clone,
    V: core::fmt::Debug + Clone,
    S: BuildHasher + Clone,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LockedChainMap")
            .field("maps", &self.maps)
            .finish()
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
        let chain_map = LockedChainMap::new(test_map);

        assert!(chain_map.maps.len() > 0);
        assert_eq!(chain_map.maps[0].get("test"), Some(&1));
    }

    #[test]
    fn initialization_default() {
        let chain_map: LockedChainMap<(), ()> = LockedChainMap::default();

        assert!(chain_map.maps.len() > 0);
        assert!(chain_map.maps[0].is_empty());
    }

    #[test]
    fn insert() {
        let mut chain_map = LockedChainMap::default();
        assert!(chain_map.insert("test", 1).is_none());

        assert_eq!(chain_map.maps[0].get("test"), Some(&1));
    }

    #[test]
    fn insert_at() {
        let mut chain_map = LockedChainMap::default();
        chain_map.insert("banana", "milk");
        chain_map.new_child();

        chain_map.insert_at(0, "strawberry", "soda").unwrap();
        assert_eq!(chain_map.maps[0].get("strawberry"), Some(&"soda"));
        assert_eq!(chain_map.maps[1].get("strawberry"), None);
    }

    #[test]
    #[should_panic = "IndexOutOfRange"]
    fn insert_at_out_of_bounds() {
        let mut chain_map = LockedChainMap::default();
        chain_map.insert("banana", "milk");
        chain_map.new_child();

        chain_map.insert_at(37, "strawberry", "soda").unwrap();
    }

    #[test]
    fn get() {
        let mut chain_map = LockedChainMap::default();
        chain_map.insert("test", 1);

        assert_eq!(chain_map.get(&"test"), Some(&1));
    }

    #[test]
    fn get_none() {
        let chain_map: LockedChainMap<&str, ()> = LockedChainMap::default();
        assert_eq!(chain_map.get(&"test"), None);
    }

    #[test]
    fn get_mut() {
        let mut chain_map = LockedChainMap::default();
        chain_map.insert("test", 1);

        let test_value = chain_map.get_mut(&"test");
        assert_eq!(test_value, Some(&mut 1));
        *test_value.unwrap() += 1;
        let changed = chain_map.get(&"test");
        assert_eq!(changed, Some(&2));
    }

    #[test]
    fn get_mut_outer() {
        let mut chain_map = LockedChainMap::default();
        chain_map.insert("outer", 1);
        chain_map.new_child();
        chain_map.insert("inner", 2);
        let ret = chain_map.get_mut("outer").unwrap();
        *ret += 9000;

        let changed = chain_map.get(&"outer");
        assert_eq!(changed, Some(&9001));
    }

    #[test]
    fn index() {
        let mut chain_map = LockedChainMap::default();
        chain_map.insert("test", 1);

        assert_eq!(chain_map[&"test"], 1);
    }

    #[test]
    fn new_child() {
        let mut chain_map = LockedChainMap::default();
        chain_map.insert("test", 1);
        chain_map.new_child();
        assert!(chain_map.maps.len() > 1);
    }

    #[test]
    fn scopes() {
        let mut chain_map = LockedChainMap::default();
        chain_map.insert("x", 0);
        chain_map.insert("y", 2);
        chain_map.new_child();
        chain_map.insert("x", 1);
        assert_eq!(chain_map.get("x"), Some(&1));
        assert_eq!(chain_map.get("y"), Some(&2));
    }

    #[test]
    fn remove_child() {
        let mut chain_map = LockedChainMap::default();
        chain_map.insert("x", 0);
        chain_map.insert("y", 2);
        chain_map.new_child();
        chain_map.insert("x", 1);
        let ret = chain_map.remove_child().unwrap();
        assert_eq!(ret.get("x"), Some(&1));
        assert_eq!(chain_map.get("x"), Some(&0));
    }

    #[test]
    fn remove_child_length_1() {
        let mut chain_map = LockedChainMap::default();
        chain_map.insert("x", 0);
        let _ = chain_map.remove_child();
        assert_eq!(chain_map.get("x"), None);
        assert!(chain_map.maps.len() == 1);
    }

    #[test]
    fn has_at_exists() {
        let mut chain_map = LockedChainMap::default();
        chain_map.insert("x", 0);

        assert!(chain_map.has_at(0, &"x"));
    }

    #[test]
    fn has_at_doesnt_exist() {
        let chain_map: LockedChainMap<&str, ()> = LockedChainMap::default();

        assert!(!chain_map.has_at(11, &"x"));
    }

    #[test]
    fn last_has_true() {
        let mut chain_map = LockedChainMap::default();
        chain_map.insert("x", 0);
        chain_map.new_child();
        chain_map.insert("y", 1);

        assert!(chain_map.last_has(&"y"));
    }

    #[test]
    fn last_has_false() {
        let mut chain_map = LockedChainMap::default();
        chain_map.insert("x", 0);
        chain_map.new_child();
        chain_map.insert("y", 1);

        assert!(!chain_map.last_has(&"x"));
    }

    #[test]
    fn child_len() {
        let mut chain_map: LockedChainMap<&str, ()> = LockedChainMap::default();
        assert_eq!(chain_map.child_len(), 1);

        for i in 2..100 {
            chain_map.new_child();
            assert_eq!(chain_map.child_len(), i);
        }
    }

    #[test]
    fn get_before_exists() {
        let mut chain_map = LockedChainMap::default();
        chain_map.insert("test", 1);
        chain_map.new_child();
        chain_map.insert("test", 2);

        assert_eq!(chain_map.get_before(1, &"test"), Some(&1));
    }

    #[test]
    fn get_before_mut_exists() {
        let mut chain_map = LockedChainMap::default();
        chain_map.insert("test", 1);
        chain_map.new_child();
        chain_map.insert("test", 2);

        let test_value = chain_map.get_before_mut(1, &"test");
        assert_eq!(test_value, Some(&mut 1));
        *test_value.unwrap() += 2;
        let changed = chain_map.get_before(1, &"test");
        assert_eq!(changed, Some(&3));
        let child = chain_map.get("test");
        assert_eq!(child, Some(&2));
    }

    #[test]
    fn get_last_index_exists() {
        let mut chain_map = LockedChainMap::default();
        chain_map.insert("test1", 1);
        chain_map.new_child();
        chain_map.insert("test2", 2);

        assert_eq!(chain_map.get_last_index("test1"), Some(0));
        assert_eq!(chain_map.get_last_index("test2"), Some(1));
    }

    #[test]
    fn get_last_index_doesnt_exist() {
        let mut chain_map = LockedChainMap::default();
        chain_map.insert("test1", 1);
        chain_map.new_child();
        chain_map.insert("test2", 2);

        assert_eq!(chain_map.get_last_index("shmee"), None);
    }
}
