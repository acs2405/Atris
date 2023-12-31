pub mod standard;
pub mod rock;

// use crate::figure::algebra::IVector;

use core::fmt::Debug;
use std::collections::HashMap;

// use super::Block;

/// This `trait` represents the behaviour of every `Block`, depending on the `Block`'s type and state (and global state).
pub trait BlockType: Debug {
    // const NAME: str;
    /// Returns the identifier (an unique name) of the `BlockType` object. The value returned must be constant all the time.
    fn id(&self) -> &'static str;

    // fn on_create(&self, e: Event);
    // fn on_damage(&self, e: Event);
    // fn new_block(&self) -> Block<Self> {
    //     Block::new(self)
    // }
}

// struct Event<'b, 'bt, 'game> {
//     target: &'b mut Block<'bt>,
//     pos: IVector,
//     grid: &'game mut Grid,
//     ctx: &'game mut GameContext,
// }
// e.preventDefault()?

/// Maps `BlockType` objects with their `id`.
pub struct BlockTypes {
    map: HashMap<&'static str, Box<dyn BlockType>>,
}

impl BlockTypes {
    /// Creates a new empty map.
    pub fn new() -> Self { Self { map: HashMap::new() } }

    /// Creates a new map and adds to it all the statically known `BlockType`s.
    /// These `BlockType`s can be retrieved by their `id` with the `get` method.
    pub fn initialize() -> Self {
        let mut bts = Self { map: HashMap::new() };
        bts.add(standard::StandardType{});
        bts.add(rock::RockType{});
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