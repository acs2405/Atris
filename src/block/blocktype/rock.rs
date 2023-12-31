use super::BlockType;
// use crate::block::Block;
// use crate::block::BitRange;

#[derive(Debug)]
pub struct RockType {}

impl RockType {
    // const BREAKING_STATE: BitRange = BitRange{pos:0, len:2};
    // const STATE_INITIAL: u64 = 0b00u64;
    // const STATE_BROKEN: u64 = 0b01u64;
}

impl BlockType for RockType {
    // const NAME: str = "Rock";
    fn id(&self) -> &'static str { "Rock" }

    // fn on_create(&self, e: Event) {}

    // fn on_damage(&self, e: Event) {
    //     let b = e.target;
    //     if b.state.get(Self::BREAKING_STATE) >= Self::STATE_BROKEN {
    //         e.signal_queue.signal(Signals::Destroy(b));
    //     } else {
    //         b.state.set(b.state.get(BREAKING_STATE) + 1)
    //     }
    // }
}