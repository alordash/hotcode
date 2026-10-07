use indexmap::IndexMap;
use std::hash::Hash;
use std::sync::RwLock;

// Map that can only return and insert values. Inserted values can not be removed or updated.
pub struct GrowMap<K, V> {
    inner: RwLock<IndexMap<K, V>>,
}

impl<K, V> GrowMap<K, V> {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(IndexMap::new()),
        }
    }
}

// SAFETY: inner map is accessible only through `RwLock`.
unsafe impl<K, V> Send for GrowMap<K, V> {}
// SAFETY: inner map is accessible only through `RwLock`.
unsafe impl<K, V> Sync for GrowMap<K, V> {}

impl<K, V> GrowMap<K, V>
where
    K: Eq + Hash,
{
    pub fn get_or_insert_with<F: FnOnce() -> V>(&self, key: K, default: F) -> &V {
        if let Some(existing) = self.get(&key) {
            return existing;
        }
        let mut write = self.inner.write().unwrap();
        let new_value = default();
        let created = write.entry(key).insert_entry(new_value);
        // SAFETY: created entries can not be removed or changed due to this struct's API, so it
        // is safe to treat references to its entries as having same lifetime as `&self`.
        // `IndexMap` also guarantees that order of entries will not change, and they won't be
        // moved.
        return unsafe { core::mem::transmute::<&V, &V>(created.get()) };
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let read = self.inner.read().unwrap();
        if let Some(existing) = read.get(key) {
            // SAFETY: created entries can not be removed or changed due to this struct's API, so it
            // is safe to treat references to its entries as having same lifetime as `&self`.
            // `IndexMap` also guarantees that order of entries will not change, and they won't be
            // moved.
            return Some(unsafe { core::mem::transmute::<&V, &V>(existing) });
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use automock::AsTimes;

    type Key = usize;
    type Value = String;

    #[test]
    fn get_or_insert_with_NoEntry_CreatesUsingFactory() {
        // Arrange
        let grow_map = GrowMap::<Key, Value>::new();
        let key: Key = 123;
        let new_value = "quo vadis".to_owned();
        factory::setup().returns(new_value.clone());

        // Act
        let result = grow_map.get_or_insert_with(key, factory);

        // Assert
        assert_eq!(result, &new_value);
        factory::received::<Value>(1.time());
    }

    #[test]
    fn get_or_insert_with_ExistingEntry_ReturnsExistingWithoutUsingFactory() {
        // Arrange
        let grow_map = GrowMap::<Key, Value>::new();
        let key: Key = 123;
        let existing_value = "quo vadis".to_owned();
        {
            let mut grow_map_write = grow_map
                .inner
                .write()
                .expect("Unable to `write` to `GrowMap.inner`");
            grow_map_write.insert(key, existing_value.clone());
        }

        // Act
        let result = grow_map.get_or_insert_with(key, factory);

        // Assert
        assert_eq!(result, &existing_value);
        factory::received_nothing::<Value>();
    }

    #[test]
    fn get_NoEntry_ReturnsNone() {
        // Arrange
        let grow_map = GrowMap::<Key, Value>::new();
        let key: Key = 123;

        // Act
        let result = grow_map.get(&key);

        // Assert
        assert_eq!(result, None);
    }

    #[test]
    fn get_WithEntry_ReturnsSome() {
        // Arrange
        let grow_map = GrowMap::<Key, Value>::new();
        let key: Key = 123;
        let existing_value = "quo vadis".to_owned();
        {
            let mut grow_map_write = grow_map
                .inner
                .write()
                .expect("Unable to `write` to `GrowMap.inner`");
            grow_map_write.insert(key, existing_value.clone());
        }

        // Act
        let result = grow_map.get(&key);

        // Assert
        assert_eq!(result, Some(&existing_value));
    }

    #[automock::mock]
    fn factory<V>() -> V {
        unreachable!()
    }
}
