use std::collections::HashMap;

use rand::{rngs::ThreadRng, thread_rng, Rng};
use uuid::Uuid;

use crate::{gridsession::GridSession, block::blocktypes::BlockTypes, figure::algebra::UVector};

#[derive(Debug)]
pub struct Player {
    id: usize,
    active: bool,
}

impl Player {
    pub fn new(player_id: usize) -> Self {
        Self {
            id: player_id,
            active: true,
        }
    }
}

#[derive(Debug)]
pub struct AtrisSession<'a> {
    uuid: Uuid,
    rng: ThreadRng,
    players: Vec<Player>,
    grids: HashMap<usize, GridSession<'a>>,
    block_types: BlockTypes,
}

impl<'a> AtrisSession<'a> {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            rng: thread_rng(),
            players: Vec::new(),
            grids: HashMap::new(),
            block_types: BlockTypes::base_types(),
        }
    }

    pub fn create_grid(&mut self, grid_bounds: UVector) {
        let mut grid_session_id: usize;
        loop {
            grid_session_id = self.rng.gen_range(1..100);
            if !self.grids.contains_key(&grid_session_id) {
                break;
            }
        }
        self.grids.insert(grid_session_id, GridSession::new(grid_session_id, grid_bounds));
    }

    pub fn create_player(&mut self) {
        let player_id = self.players.len();
        self.players.push(Player::new(player_id));
        // self.grids.create_figure(player_id);
    }
}