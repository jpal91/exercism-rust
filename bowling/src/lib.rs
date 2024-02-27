#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum SubF {
    One,
    Two,
    Final(u8),
}

#[derive(Clone, Debug)]
enum Frame {
    Strike,
    Spare(u16),
    Open(u16, u16),
    Final(u16, u16, u16),
}

pub struct BowlingGame {
    frames: Vec<Frame>,
    n: usize,
    sub: SubF,
    rem: u16,
    game_over: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        let mut frames = vec![Frame::Open(0, 0); 9];
        frames.push(Frame::Final(0, 0, 0));

        Self {
            frames,
            n: 0,
            sub: SubF::One,
            rem: 10,
            game_over: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > self.rem {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.game_over {
            return Err(Error::GameComplete);
        }

        self.rem -= pins;

        match (self.sub, self.rem) {
            (SubF::One, 0) => {
                self.frames[self.n] = Frame::Strike;
                self.n += 1;
                self.rem = 10;
            }
            (SubF::One, _) => {
                if let Frame::Open(_, b) = self.frames[self.n] {
                    self.frames[self.n] = Frame::Open(pins, b)
                }
                self.sub = SubF::Two
            }
            (SubF::Two, 0) => {
                if let Frame::Open(a, _) = self.frames[self.n] {
                    self.frames[self.n] = Frame::Spare(a)
                }
                self.sub = SubF::One;
                self.n += 1;
                self.rem = 10;
            }
            (SubF::Two, _) => {
                if let Frame::Open(a, _) = self.frames[self.n] {
                    self.frames[self.n] = Frame::Open(a, pins)
                }
                self.sub = SubF::One;
                self.n += 1;
                self.rem = 10;
            }
            (SubF::Final(n), rem) => {
                self.frames[self.n] = match (n, &self.frames[self.n]) {
                    (0, Frame::Final(_, _, _)) => {
                        if rem == 0 {
                            self.rem = 10
                        }
                        Frame::Final(pins, 0, 0)
                    }
                    (1, Frame::Final(a, _, _)) => {
                        if rem > 0 && a != &10 {
                            self.game_over = true;
                        } else if rem == 0 {
                            self.rem = 10;
                        }
                        Frame::Final(*a, pins, 0)
                    }
                    (2, Frame::Final(a, b, _)) => {
                        self.game_over = true;
                        Frame::Final(*a, *b, pins)
                    }
                    _ => unimplemented!(),
                };

                self.sub = SubF::Final(n + 1);
            }
        }

        if self.n == 9 && self.sub == SubF::One {
            self.sub = SubF::Final(0);
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.game_over {
            return None;
        }

        let frames: &[Frame] = &self.frames;
        let mut score = 0;

        for i in 0..10 {
            match frames[i] {
                Frame::Strike => {
                    score += 10;
                    let mut idx = i + 1;

                    while idx < 10 && idx <= i + 2 {
                        match frames[idx] {
                            Frame::Open(a, b) | Frame::Final(a, b, _) if idx < i + 2 => {
                                score += a + b;
                                break;
                            }
                            Frame::Open(a, _) | Frame::Final(a, _, _) => {
                                score += a;
                                break;
                            }
                            Frame::Spare(a) => {
                                score += a + 10;
                                break;
                            }
                            Frame::Strike => score += 10,
                        }
                        idx += 1;
                    }
                }
                Frame::Spare(_) => {
                    score += 10;

                    if i + 1 < 10 {
                        let add = match frames[i + 1] {
                            Frame::Open(a, _) => a,
                            Frame::Spare(a) => a,
                            Frame::Final(a, _, _) => a,
                            Frame::Strike => 10,
                        };
                        score += add;
                    }
                }
                Frame::Open(a, b) => score += a + b,
                Frame::Final(a, b, c) => score += a + b + c,
            }
        }

        Some(score)
    }
}
