#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: [u8; 21],
    current_roll: usize,
    skip: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            rolls: [0; 21],
            current_roll: 0,
            skip: 0,
        }
    }

    pub fn roll(&mut self, pins: u8) -> Result<(), Error> {
        let valid = self.is_valid_roll(pins);
        if valid.is_err() {
            return valid;
        }

        self.rolls[self.current_roll] = pins;

        if self.is_strike(self.current_roll) && !self.is_last_frame() {
            self.skip += 1;
        }

        self.current_roll += 1;

        Ok(())
    }

    pub fn score(&self) -> Option<u32> {
        if !self.is_game_done() {
            return None;
        }

        let mut result = 0;
        let mut frame_index = 0;

        for _ in 0..10 {
            if self.is_strike(frame_index) {
                result += 10 + self.strike_bonus(frame_index);
                frame_index += 1;
            } else if self.is_spare(frame_index) {
                result += 10 + self.spare_bonus(frame_index);
                frame_index += 2;
            } else {
                result += self.sum_frame(frame_index);
                frame_index += 2;
            }
        }

        Some(result)
    }

    fn is_strike(&self, frame_index: usize) -> bool {
        self.rolls[frame_index] == 10
    }

    fn strike_bonus(&self, frame_index: usize) -> u32 {
        (self.rolls[frame_index + 1] as u32) + (self.rolls[frame_index + 2] as u32)
    }

    fn is_spare(&self, frame_index: usize) -> bool {
        self.rolls[frame_index] + self.rolls[frame_index + 1] == 10
    }

    fn spare_bonus(&self, frame_index: usize) -> u32 {
        self.rolls[frame_index + 2] as u32
    }

    fn sum_frame(&self, frame_index: usize) -> u32 {
        (self.rolls[frame_index] as u32) + (self.rolls[frame_index + 1] as u32)
    }

    fn is_valid_roll(&self, pins: u8) -> Result<(), Error> {
        if self.is_game_done() {
            return Err(Error::GameComplete);
        }

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.current_roll >= 1 && self.rolls[self.current_roll - 1] + pins > 10 {
            if self.current_roll >= 2 {
                if !self.is_strike(self.current_roll - 1)
                    && !self.is_spare(self.current_roll - 2)
                    && !(self.sum_frame(self.current_roll - 2) < 10)
                {
                    return Err(Error::NotEnoughPinsLeft);
                }
            } else if !self.is_strike(self.current_roll - 1) {
                return Err(Error::NotEnoughPinsLeft);
            }
        }

        Ok(())
    }

    fn is_game_done(&self) -> bool {
        let offset_index = self.current_roll + self.skip;
        if offset_index < 20 {
            return false;
        }

        let a = self.rolls[18 - self.skip];
        let b = self.rolls[19 - self.skip];

        (a + b < 10 && offset_index == 20) || offset_index == 21
    }

    fn is_last_frame(&self) -> bool {
        self.current_roll + self.skip >= 18
    }
}
