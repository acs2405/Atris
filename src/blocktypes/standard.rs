use crate::block::BlockType;
// use crate::block::BitRange;

#[derive(Debug)]
pub struct StandardType {}

impl BlockType for StandardType {
    // const NAME: str = "Standard";
    fn id(&self) -> &'static str { "Standard" }

    // fn on_create(&self, e: Event) {}
    // fn on_damage(&self, e: Event) {}
}