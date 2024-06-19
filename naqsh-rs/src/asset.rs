#![allow(dead_code)]
use std::collections::HashMap;

use crate::image::Image;

enum Asset {
    Image
}

/// Assets Map.
pub struct Assets {
    assets: HashMap<String, Asset>,
    images: HashMap<String, Image>,
}

/// Asset Map Methods
pub trait AssetsMethods<T> {
    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, None is returned.
    fn insert(&mut self, k: &str, v: T) -> Option<T>;
    /// Returns a reference to the value corresponding to the key.
    fn get(&self, k: &str) -> Option<&T>;
    /// Returns a mutable reference to the value corresponding to the key.
    fn get_mut(&mut self, k: &str) -> Option<&mut T>;
    /// Removes a key from the map, returning the value at the key if the key was previously in the map.
    fn remove(&mut self, k: &str) -> Option<T>;
}

impl AssetsMethods<Image> for Assets {
    fn insert(&mut self, k: &str, v: Image) -> Option<Image> {
        self.assets.insert(k.to_string(), Asset::Image);
        self.images.insert(k.to_string(), v)
    }
    fn get(&self, k: &str) -> Option<&Image> {
        self.images.get(k)
    }
    fn get_mut(&mut self, k: &str) -> Option<&mut Image> {
        self.images.get_mut(k)
    }
    fn remove(&mut self, k: &str) -> Option<Image> {
        self.images.remove(k)
    }
}