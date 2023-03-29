mod frame;
mod tests;

use frame::frame::*;

pub struct BowlingGame {
    frames: Frame,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: Frame::new(),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        self.frames.roll(pins)
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames.iter().last().unwrap().is_final_frame()
            && self.frames.iter().last().unwrap().is_done()
        {
            return Some(self.frames.iter().map(|frame| frame.score()).sum());
        }
        None
    }
}
