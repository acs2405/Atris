// use crate::figure::algebra::IVector;
use crate::block::BlockType;

use std::collections::HashMap;

/// Maps `BlockType` objects with their `id`.
#[derive(Debug)]
pub struct BlockTypes {
    map: HashMap<&'static str, Box<dyn BlockType>>,
}

impl BlockTypes {
    /// Creates a new empty map.
    pub fn new() -> Self { Self { map: HashMap::new() } }

    /// Creates a new map and adds to it all the statically known `BlockType`s.
    /// These `BlockType`s can be retrieved by their `id` with the `get` method.
    pub fn base_types() -> Self {
        let mut bts = Self { map: HashMap::new() };
        bts.add(super::standard::StandardType{});
        bts.add(super::rock::RockType{});
        // Add new `BlockType` structs instances here
        bts
    }

    /// Returns `Some<bt>` if some `bt` (`BlockType` object) exists in the map with that `id`. Otherwise, it returns `None`.
    pub fn get(&self, id: &str) -> Option<&dyn BlockType> {
        Some(self.map.get(id)?.as_ref())
    }

    /// Adds a `BlockType` object to the list, using its `id()` method as key. If another one with the same name already
    /// exists in the map, the new one replaces it.
    /// The method returns `Some<bt>` if some `bt` object existed before and is being replaced. Otherwise, it returns `None`.
    fn add<T: BlockType + 'static>(&mut self, bt: T) -> Option<Box<dyn BlockType>> {
        self.map.insert(bt.id(), Box::new(bt))
    }
}