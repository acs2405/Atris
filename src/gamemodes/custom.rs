use rand::Rng;

use crate::{gamemode::GameMode, piece::{shape::Shapes, Piece}, block::BlockType, blocktypes::standard::StandardType};

#[derive(Debug)]
pub struct CustomMode {
    shapes: Shapes,
    block_types: Vec<Box<dyn BlockType>>,
    configurations: Vec<PieceConfiguration>,
    total_weight: u32,
}

impl CustomMode {
    pub fn new(block_types: Vec<Box<dyn BlockType>>, configurations: Vec<PieceConfiguration>) -> Self {
        Self {
            shapes: Shapes::new(),
            block_types: block_types,
            configurations: configurations,
            total_weight: 0,
        }
    }
    
    fn next_configuration<R: Rng>(&self, rng: &mut R) -> &PieceConfiguration {
        let n = rng.next_u32();
        for conf in self.configurations.iter() {
            if n < conf.weight {return conf;}
        };
        unreachable!()
    }
}

impl<R: Rng> GameMode<R> for CustomMode {
    fn initialize(&mut self) {
        // If no block types are specified, just use the standard block type.
        if self.block_types.is_empty() {
            self.block_types.push(Box::new(StandardType{}));
        }

        // If no configurations are specified, just use one for each block type.
        if self.configurations.is_empty() {
            self.configurations = (0..self.block_types.len()).map(|bt| {
                PieceConfiguration {
                    block_type: bt,
                    n_blocks: 4,
                    weight: 1,
                }
            }).collect();
        }

        // Sort configurations by weight desc as the bigger it is, the more likely to pick.
        self.configurations
            .sort_by(|conf1, conf2| {
                conf2.weight.cmp(&conf1.weight)
            });

        self.configurations
            .iter_mut()
            .filter(|conf| {
                // Discard invalid or unweighted configurations
                conf.weight > 0 && conf.n_blocks > 0 && conf.block_type < self.block_types.len()
            }).for_each(|conf| {
                // Accumulate weights so it's easier to pick a random configuration.
                self.total_weight += conf.weight;
                conf.weight = self.total_weight;
            });

        // Non referenced block types are not discarded.

        let max_n_blocks = 
            match self.configurations.iter().map(|conf| conf.n_blocks).max() {
                Some(n_blocks) => n_blocks,
                None => 4,
            };
        self.shapes.gen_until(max_n_blocks);
        
        // self.total_weight = self.configurations.iter().map(|conf| conf.weight).sum();
    }

    fn next_piece(&self, rng: &mut R) -> Piece {
        let conf = self.next_configuration(rng);
        Piece::uniform(self.block_types[conf.block_type].as_ref(), self.shapes.random(conf.n_blocks, rng))
    }
}

#[derive(Debug)]
pub struct PieceConfiguration {
    block_type: usize,
    // block_state: State<u64>,
    n_blocks: usize,
    weight: u32,
}