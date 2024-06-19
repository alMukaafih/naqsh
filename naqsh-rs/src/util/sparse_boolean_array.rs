#![allow(dead_code)]
/// SparseBooleanArrays map integers to booleans.
/// Unlike a normal array of booleans
/// there can be gaps in the indices.  It is intended to be more memory efficient
/// than using a HashMap to map Integers to Booleans, both because it avoids
/// auto-boxing keys and values and its data structure doesn't rely on an extra entry object
/// for each mapping.
///
/// <p>Note that this container keeps its mappings in an array data structure,
/// using a binary search to find keys.  The implementation is not intended to be appropriate for
/// data structures
/// that may contain large numbers of items.  It is generally slower than a traditional
/// HashMap, since lookups require a binary search and adds and removes require inserting
/// and deleting entries in the array.  For containers holding up to hundreds of items,
/// the performance difference is not significant, less than 50%.</p>
///
/// <p>It is possible to iterate over the items in this container using
/// {@link #keyAt(int)} and {@link #valueAt(int)}. Iterating over the keys using
/// <code>keyAt(int)</code> with ascending values of the index will return the
/// keys in ascending order, or the values corresponding to the keys in ascending
/// order in the case of <code>valueAt(int)</code>.</p>
pub struct SparseBooleanArray {
    m_keys: Vec<i32>,
    m_values: Vec<bool>,
    m_size: usize
}

impl SparseBooleanArray {
    /// Creates a new SparseBooleanArray containing no mappings that will not
    /// require any additional memory allocation to store the specified
    /// number of mappings.  If you supply an initial capacity of 0, the
    /// sparse array will be initialized with a light-weight representation
    /// not requiring any additional array allocations.
    pub fn new(initial_capacity: usize) -> Self {
        Self {
            m_keys: Vec::with_capacity(initial_capacity),
            m_values: Vec::with_capacity(initial_capacity),
            m_size: 0
        }
    }

    /// Gets the boolean mapped from the specified key, or <code>false</code>
    /// if no such mapping has been made.
    pub fn get_or_false(&self, key: i32) -> bool {
        self.get(key, false)
    }

    /// Gets the boolean mapped from the specified key, or the specified value
    /// if no such mapping has been made.
    pub fn get(&self, key: i32, value_if_key_not_found: bool) -> bool {
        let i = self.m_keys.binary_search(&key);

        if let Ok(i) = i {
            self.m_values[i]
        } else {
            value_if_key_not_found
        }
    }

    /// Removes the mapping from the specified key, if there was any.
    pub fn delete(&mut self, key: i32) {
        let i = self.m_keys.binary_search(&key);

        if let Ok(i) = i {
            self.m_keys.remove(i);
            self.m_values.remove(i);
            self.m_size -= 1;
        }
    }

    /// Adds a mapping from the specified key to the specified value,
    /// replacing the previous mapping from the specified key if there
    /// was one.
    pub fn put(&mut self, key: i32, value: bool) {
        let i = self.m_keys.binary_search(&key);

        match i {
            Ok(i) => {
                self.m_values[i] = value
            }
            Err(i) => {
                self.m_keys.insert(i, key);
                self.m_values.insert(i,value);
                self.m_size += 1;
            }
        }
    }

    /// Removes all key-value mappings from this SparseBooleanArray.
    pub fn clear(&mut self) {
        self.m_keys.clear();
        self.m_values.clear();
        self.m_size = 0;
    }

    /// Puts a key/value pair into the array, optimizing for the case where
    /// the key is greater than all existing keys in the array.
    pub fn append(&mut self, key: i32, value: bool) {
        if self.m_size != 0 && key <= self.m_keys[self.m_size - 1] {
            self.put(key, value);
            return;
        }
        self.m_keys.push(key);
        self.m_values.push(value);
        self.m_size += 1;
    }
}

impl Default for SparseBooleanArray {
    /// Creates a new SparseBooleanArray containing no mappings.
    fn default() -> Self {
       Self::new(0)
    }
}