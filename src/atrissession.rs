use std::{collections::HashMap, thread::{Thread, JoinHandle}, rc::Rc};

use crossbeam::channel::{Receiver, unbounded, Sender};
use futures::executor::ThreadPool;
use rand::{rngs::StdRng, Rng, SeedableRng};
use uuid::Uuid;

use crate::{gridsession::GridSession, algebra::UVector, gamemode::GameMode, block::state::State};

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
pub struct AtrisSession {
    uuid: Uuid,
    players: Vec<Player>,
    // Configurations:
    game_mode: Box<dyn GameMode<StdRng>>,
    tps: u64, // Ticks per Second, for the loop
    // State:
    state: State<u64>,
    rng: StdRng,
    // Thread pool and events channels:
    thread_pool: ThreadPool,
    grids: HashMap<usize, Receiver<()>>,
    sender: Sender<()>,
    receiver: Receiver<()>,
}

impl AtrisSession {
    pub fn new(game_mode: Box<dyn GameMode<StdRng>>, thread_pool: ThreadPool) -> Self {
        let (sender, receiver) = unbounded();
        Self {
            uuid: Uuid::new_v4(),
            players: Vec::new(),
            game_mode: game_mode,
            tps: 30,
            state: State::new(),
            rng: StdRng::from_entropy(), //thread_rng(),
            thread_pool: thread_pool,
            grids: HashMap::new(),
            sender: sender,
            receiver: receiver,
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
        let (grid_sender, grid_receiver) = unbounded(); // For grid events
        let game_receiver = self.receiver.clone();
        self.grids.insert(grid_session_id, grid_receiver);
        let tps = self.tps; // And other configuration (make struct)
        self.thread_pool.spawn_ok(async move {
            let mut grid_session = GridSession::new(grid_session_id, grid_bounds, grid_sender, game_receiver, tps);
            grid_session.await_until_start();
            // grid_session.start(); // Wait until start event (make enum)
        });
        self.sender.send(()); // Send GridAdded(Receiver)
        // let tps = self.tps;
        // let block_types = &self.block_types;
        // self.grids.insert(grid_session_id, thread);
    }

    pub fn create_player(&mut self) {
        let player_id = self.players.len();
        self.players.push(Player::new(player_id));
        // self.grids.create_figure(player_id);
    }

    pub fn start(&mut self) {
        // let grid_session = self.grids.remove(&3).unwrap();
        // self.thread_pool.spawn_ok(async move { grid_session.start(); });
        for grid_session in self.grids.values_mut() {
            // let thread = thread::spawn(|| {
                // self.thread_pool.spawn_ok(async {
                //     grid_session.start();
                // });
                // self.grids[&grid_session_id].run()
            // });
        }
    }
}