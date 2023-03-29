use crate::Error::{GameComplete, NotEnoughPinsLeft};

mod tests;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Frame,
}

struct Frame {
    index: u16,
    first_roll: u16,
    second_roll: u16,
    next_frame: Option<Box<Frame>>,
}

struct Iter<'a> {
    next: Option<&'a Frame>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Frame;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|frame| {
            self.next = frame.next_frame.as_deref();
            frame
        })
    }
}

impl Frame {
    const STRIKE: u16 = 10;
    const NOT_ROLLED: u16 = 11;
    const HALF_FRAME: u16 = 12;

    fn new() -> Frame {
        Frame {
            index: 0,
            first_roll: Frame::NOT_ROLLED,
            second_roll: Frame::NOT_ROLLED,
            next_frame: None,
        }
    }
    fn next_from_roll(&self, roll: u16) -> Frame {
        Frame {
            index: self.index + 1,
            first_roll: roll,
            second_roll: if self.index == 9 && !self.is_strike() {
                Frame::HALF_FRAME
            } else {
                Frame::NOT_ROLLED
            },
            next_frame: None,
        }
    }

    fn iter(&self) -> Iter {
        Iter { next: Some(self) }
    }

    fn next(&self) -> &Frame {
        self.next_frame.as_deref().unwrap()
    }

    fn score(&self) -> u16 {
        return if self.index == 10 {
            0
        } else if self.is_strike() {
            if self.next().is_strike() {
                if self.next().is_final_frame() {
                    return self.first_roll + self.next().first_roll + self.next().second_roll;
                }
                return self.first_roll + self.next().first_roll + self.next().next().first_roll;
            }
            return self.first_roll + self.next().first_roll + self.next().second_roll;
        } else if self.is_spare() {
            self.first_roll + self.second_roll + self.next().first_roll
        } else {
            self.first_roll + self.second_roll
        };
    }

    fn roll(&mut self, roll: u16) -> Result<(), Error> {
        if roll > Frame::STRIKE {
            Err(NotEnoughPinsLeft)
        } else if self.is_done() {
            if self.is_final_frame() {
                return Err(GameComplete);
            }
            if self.next_frame.is_some() {
                return self.next_frame.as_deref_mut().unwrap().roll(roll);
            }
            self.next_frame = Some(Box::new(self.next_from_roll(roll)));
            return Ok(());
        } else if self.first_roll == Frame::NOT_ROLLED {
            self.first_roll = roll;
            return Ok(());
        } else {
            if ((self.is_final_frame() && !self.is_strike()) || !self.is_final_frame())
                && self.first_roll + roll > Frame::STRIKE
            {
                return Err(NotEnoughPinsLeft);
            }
            self.second_roll = roll;
            return Ok(());
        }
    }

    fn is_done(&self) -> bool {
        self.first_roll != Frame::NOT_ROLLED
            && (self.second_roll != Frame::NOT_ROLLED
                || self.second_roll == Frame::HALF_FRAME
                || (self.is_strike() && !self.is_final_frame()))
    }

    fn is_strike(&self) -> bool {
        self.first_roll == Frame::STRIKE
    }

    fn is_spare(&self) -> bool {
        self.first_roll + self.second_roll == Frame::STRIKE
    }

    fn is_final_frame(&self) -> bool {
        (self.index == 9 && !(self.is_strike() || self.is_spare())) || self.index == 10
    }
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
