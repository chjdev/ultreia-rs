use std::collections::HashMap;
use std::hash::Hash;
use std::mem::swap;
use std::ops::DerefMut;
use std::sync::{Arc, RwLock};

pub struct StackedLRU<K: Hash + Eq + Clone, V> {
    promotion_layer: RwLock<HashMap<K, Arc<V>>>,
    basic_layer: RwLock<HashMap<K, Arc<V>>>,
    demotion_layer: RwLock<HashMap<K, Arc<V>>>,
    capacity: usize,
}

impl<K: Hash + Eq + Clone, V> StackedLRU<K, V> {
    pub fn new(capacity: usize) -> Self {
        StackedLRU {
            promotion_layer: RwLock::new(HashMap::new()),
            basic_layer: RwLock::new(HashMap::new()),
            demotion_layer: RwLock::new(HashMap::with_capacity(0)),
            capacity,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.promotion_layer.read().unwrap().len()
            + self.basic_layer.read().unwrap().len()
            + self.demotion_layer.read().unwrap().len()
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.promotion_layer.read().unwrap().contains_key(key)
            || self.basic_layer.read().unwrap().contains_key(key)
            || self.demotion_layer.read().unwrap().contains_key(key)
    }

    // todo generalize with arbitrary layer depth using a queue
    pub fn reference(&self, key: K, gen_value: impl FnOnce(&K) -> V) -> Arc<V> {
        // exists?
        {
            let promotion_layer = self.promotion_layer.read().unwrap();
            if promotion_layer.contains_key(&key) {
                return promotion_layer.get(&key).unwrap().clone();
            }
        }

        {
            let mut basic_layer = self.basic_layer.write().unwrap();
            let mut promotion_layer = self.promotion_layer.write().unwrap();
            if basic_layer.contains_key(&key) {
                // move up
                let value = basic_layer.remove(&key).unwrap();
                promotion_layer.insert(key.clone(), value);
                return promotion_layer.get(&key).unwrap().clone();
            }
        }
        {
            let mut demotion_layer = self.demotion_layer.write().unwrap();
            let mut basic_layer = self.basic_layer.write().unwrap();
            if demotion_layer.contains_key(&key) {
                // move up
                let value = demotion_layer.remove(&key).unwrap();
                basic_layer.insert(key.clone(), value);
                return basic_layer.get(&key).unwrap().clone();
            }
        }
        // insert new value
        let value: V = gen_value(&key);
        // still space?
        if self.len() < self.capacity() {
            let mut promotion_layer = self.promotion_layer.write().unwrap();
            promotion_layer.insert(key.clone(), Arc::new(value));
            return promotion_layer.get(&key).unwrap().clone();
        }
        // egalitarian demotion layer
        {
            let mut demotion_layer = self.demotion_layer.write().unwrap();
            let mut promotion_layer = self.promotion_layer.write().unwrap();
            if demotion_layer.len() > 1 {
                // clone trait for this because there's no nifty way to get a ref to the first key if it is the one that gets removed
                let first_key: K = demotion_layer.keys().next().unwrap().clone();
                demotion_layer.remove(&first_key);
                promotion_layer.insert(key.clone(), Arc::new(value));
                return promotion_layer.get(&key).unwrap().clone();
            }
        }
        // otherwise push down
        let mut tmp: HashMap<K, Arc<V>> = HashMap::with_capacity(self.capacity());
        {
            let mut promotion_layer = self.promotion_layer.write().unwrap();
            // swap in new layer to promotion layer
            swap(promotion_layer.deref_mut(), &mut tmp);
        }
        {
            let mut basic_layer = self.basic_layer.write().unwrap();
            // swap promotion layer into basic layer
            swap(basic_layer.deref_mut(), &mut tmp);
        }
        {
            let mut demotion_layer = self.demotion_layer.write().unwrap();
            // swap basic layer into demotion layer
            swap(demotion_layer.deref_mut(), &mut tmp);
        }
        // and try again (maybe needs an additional push down in recursion)
        self.reference(key, Self::parrot(value))
    }

    fn parrot(value: V) -> impl FnOnce(&K) -> V {
        |_key| value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Deref;

    fn gen_value(key: &usize) -> usize {
        key + 1
    }

    #[test]
    fn test_simple_insert() {
        let cache: StackedLRU<usize, usize> = StackedLRU::new(usize::max_value());
        let cached_value = cache.reference(0, gen_value);
        assert_eq!(cached_value.deref(), &1);
        assert!(cache.promotion_layer.read().unwrap().contains_key(&0));
        assert_eq!(cache.promotion_layer.read().unwrap().len(), 1);
        assert!(cache.basic_layer.read().unwrap().is_empty());
        assert!(cache.demotion_layer.read().unwrap().is_empty());
        let cached_value_same = cache.reference(0, |key| key + 1);
        assert!(Arc::<usize>::ptr_eq(&cached_value, &cached_value_same));
        let cached_value_2 = cache.reference(1, |key| key + 1);
        assert_eq!(cached_value_2.deref(), &2);
        assert!(cache.promotion_layer.read().unwrap().contains_key(&0));
        assert!(cache.promotion_layer.read().unwrap().contains_key(&1));
        assert_eq!(cache.promotion_layer.read().unwrap().len(), 2);
        assert!(cache.basic_layer.read().unwrap().is_empty());
        assert!(cache.demotion_layer.read().unwrap().is_empty());
    }

    #[test]
    fn test_insert_pushdown() {
        let cache: StackedLRU<usize, usize> = StackedLRU::new(2);

        assert_eq!(cache.reference(0, gen_value).deref(), &1);
        assert!(cache.contains_key(&0));
        assert!(cache.promotion_layer.read().unwrap().contains_key(&0));
        assert_eq!(cache.promotion_layer.read().unwrap().len(), 1);
        assert!(cache.basic_layer.read().unwrap().is_empty());
        assert!(cache.demotion_layer.read().unwrap().is_empty());

        assert_eq!(cache.reference(1, gen_value).deref(), &2);
        assert!(cache.contains_key(&0));
        assert!(cache.contains_key(&1));
        assert!(cache.promotion_layer.read().unwrap().contains_key(&0));
        assert!(cache.promotion_layer.read().unwrap().contains_key(&1));
        assert_eq!(cache.promotion_layer.read().unwrap().len(), 2);
        assert!(cache.basic_layer.read().unwrap().is_empty());
        assert!(cache.demotion_layer.read().unwrap().is_empty());

        assert_eq!(cache.reference(2, gen_value).deref(), &3);
        assert_eq!(cache.len(), 2);
        assert!(cache.contains_key(&2));
        assert!(!cache.contains_key(&0) || !cache.contains_key(&1));
        assert!(cache.promotion_layer.read().unwrap().contains_key(&2));
        assert_eq!(cache.promotion_layer.read().unwrap().len(), 1);
        assert!(cache.basic_layer.read().unwrap().is_empty());
        assert!(
            cache.demotion_layer.read().unwrap().contains_key(&1)
                || cache.demotion_layer.read().unwrap().contains_key(&0)
        );
        assert_eq!(cache.demotion_layer.read().unwrap().len(), 1);

        assert_eq!(cache.reference(3, gen_value).deref(), &4);
        assert_eq!(cache.len(), 2);
        assert!(cache.contains_key(&3));
        assert!(!cache.contains_key(&0) && !cache.contains_key(&1));
        assert!(cache.promotion_layer.read().unwrap().contains_key(&3));
        assert_eq!(cache.promotion_layer.read().unwrap().len(), 1);
        assert!(cache.basic_layer.read().unwrap().contains_key(&2));
        assert_eq!(cache.basic_layer.read().unwrap().len(), 1);
        assert!(cache.demotion_layer.read().unwrap().is_empty());
    }

    #[test]
    fn test_insert() {
        let cache: StackedLRU<usize, usize> = StackedLRU::new(4);
        let mut originals: Vec<Arc<usize>> = Vec::new();
        for i in (0 as usize)..4 {
            originals.insert(i, cache.reference(i, gen_value));
        }
        assert_eq!(cache.len(), 4);
        for i in (0 as usize)..4 {
            assert!(cache.contains_key(&i));
            assert!(cache.promotion_layer.read().unwrap().contains_key(&i));
        }
        for i in (0 as usize)..4 {
            cache.reference(i + 1, gen_value);
        }
        assert_eq!(cache.len(), 4);
        let mut counter = 0;
        for i in (0 as usize)..4 {
            if cache.contains_key(&i) {
                let same = cache.reference(i, gen_value);
                assert!(Arc::<usize>::ptr_eq(&originals[i], &same));
                counter += 1;
            }
        }
        assert!(counter >= 2);
        for i in (0 as usize)..4 {
            cache.reference(i, gen_value);
        }
        assert_eq!(cache.len(), 4);
    }
}
