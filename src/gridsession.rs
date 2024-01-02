use std::{collections::HashMap, time::{Duration, Instant}, thread::sleep};

use crossbeam::channel::{Sender, Receiver};
use rand::{rngs::ThreadRng, thread_rng};

use crate::{grid::Grid, grid::Figure, algebra::UVector, block::state::State};

#[derive(Debug)]
pub struct GridSession<'a> {
    id: usize,
    grid: Grid<'a>,
    // blocks: Vec<PositionedBlock<'a>>,
    figures: HashMap<usize, Figure<'a>>,
    // Configurations:
    tps: u64,
    // State:
    state: State<u64>,
    rng: ThreadRng, //Arc/Rc<Mutex?<StdRng>>,
    loop_timestamp: Instant,
    // Events channels:
    sender: Sender<()>,
    receiver: Receiver<()>
}

impl<'a> GridSession<'a> {
    pub fn new(grid_session_id: usize, grid_bounds: UVector, sender: Sender<()>, receiver: Receiver<()>, tps: u64) -> Self {
        Self {
            id: grid_session_id,
            grid: Grid::new(grid_bounds),
            figures: HashMap::new(),
            tps: tps,
            state: State::new(),
            rng: thread_rng(),
            loop_timestamp: Instant::now(),
            sender: sender,
            receiver: receiver,
        }
    }

    pub fn await_until_start(&mut self) {
        // Check that didn't already started
        match self.receiver.recv() {
            Ok(e) => self.start(), // GameStart or GridAdded(Receiver)
            Err(_) => (),
        };
    }

    fn get_messages(&mut self) {
        for e in self.receiver.try_iter() {
            unimplemented!();
            // One of the message types may be GridAdded(Receiver)
        };
    }

    pub fn start(&mut self) {
        let between_ticks = Duration::from_millis(1000 / self.tps);
        loop {
            self.loop_timestamp = Instant::now();
            self.game_loop();
            let timestamp = Instant::now();
            let diff = timestamp - self.loop_timestamp;
            if between_ticks > diff {
                sleep(between_ticks - diff);
            }
        }
    }

    fn game_loop(&mut self) {
        self.get_messages();
        //...
    }
}