// use crate::figure::algebra::IVector;

use core::fmt::Debug;

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