use super::BlockType;

#[derive(Debug)]
pub struct StandardType {}

impl BlockType for StandardType {
    // const NAME: str = "Standard";

    fn on_destroy(&self, _b: &crate::block::Block) {}
}