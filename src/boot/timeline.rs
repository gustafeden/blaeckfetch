/// Animation state machine: Entrance → Alive → Freeze → Done
pub enum Phase {
    Entrance,
    Alive,
    Freeze,
    Done,
}

pub struct Timeline {
    pub frame: u32,
    pub phase: Phase,
    pub freeze_start: u32,
}

// Entrance timing (frames at 30fps)
const BORDER_START: u32 = 0;
const BORDER_END: u32 = 18; // 600ms
const STARS_START: u32 = 6;
const STARS_END: u32 = 12;
const CELESTIAL_START: u32 = 12;
const CELESTIAL_END: u32 = 17;
const STATUS_START: u32 = 20;
const STATUS_END: u32 = 35;
const FOOTER_START: u32 = 30;
const ENTRANCE_DONE: u32 = 35;

// Freeze: 2 frames flash + 10 frames collapse (bottom slides up 20 rows)
const FLASH_FRAMES: u32 = 2;
const COLLAPSE_FRAMES: u32 = 10;
const FREEZE_DURATION: u32 = FLASH_FRAMES + COLLAPSE_FRAMES; // 12 frames ~400ms

impl Timeline {
    pub fn new() -> Self {
        Self {
            frame: 0,
            phase: Phase::Entrance,
            freeze_start: 0,
        }
    }

    pub fn tick(&mut self) {
        self.frame += 1;

        match self.phase {
            Phase::Entrance => {
                if self.frame >= ENTRANCE_DONE {
                    self.phase = Phase::Alive;
                }
            }
            Phase::Freeze => {
                if self.frame - self.freeze_start >= FREEZE_DURATION {
                    self.phase = Phase::Done;
                }
            }
            _ => {}
        }
    }

    pub fn trigger_freeze(&mut self) {
        if matches!(self.phase, Phase::Entrance | Phase::Alive) {
            self.phase = Phase::Freeze;
            self.freeze_start = self.frame;
        }
    }

    pub fn is_done(&self) -> bool {
        matches!(self.phase, Phase::Done)
    }

    // --- Progress getters (0.0 to 1.0) ---

    pub fn border_progress(&self) -> f32 {
        ramp(self.frame, BORDER_START, BORDER_END)
    }

    pub fn star_visibility(&self) -> f32 {
        match self.phase {
            Phase::Entrance => ramp(self.frame, STARS_START, STARS_END),
            Phase::Freeze => {
                if self.is_freeze_flash() {
                    1.0
                } else {
                    0.0 // scene content gone during collapse
                }
            }
            Phase::Done => 0.0,
            _ => 1.0,
        }
    }

    pub fn celestial_visibility(&self) -> f32 {
        match self.phase {
            Phase::Entrance => ramp(self.frame, CELESTIAL_START, CELESTIAL_END),
            Phase::Freeze => {
                if self.is_freeze_flash() {
                    1.0
                } else {
                    0.0
                }
            }
            Phase::Done => 0.0,
            _ => 1.0,
        }
    }

    pub fn status_progress(&self) -> f32 {
        match self.phase {
            Phase::Entrance => ramp(self.frame, STATUS_START, STATUS_END),
            _ => 1.0,
        }
    }

    pub fn footer_visible(&self) -> bool {
        match self.phase {
            Phase::Entrance => self.frame >= FOOTER_START,
            Phase::Alive => {
                // Blink: ~1s on, ~1s off
                ((self.frame - ENTRANCE_DONE) / 30) % 2 == 0
            }
            Phase::Freeze => true,
            Phase::Done => false,
        }
    }

    pub fn gradient_active(&self) -> bool {
        matches!(self.phase, Phase::Alive)
    }

    pub fn is_freeze_flash(&self) -> bool {
        matches!(self.phase, Phase::Freeze) && (self.frame - self.freeze_start) < FLASH_FRAMES
    }

    /// Collapse progress as 0.0..=1.0 with ease-out (fast start, slow finish).
    pub fn collapse_progress(&self) -> f32 {
        let t = match self.phase {
            Phase::Done => 1.0,
            Phase::Freeze => {
                let elapsed = self.frame - self.freeze_start;
                if elapsed < FLASH_FRAMES {
                    0.0
                } else {
                    let linear = (elapsed - FLASH_FRAMES) as f32 / COLLAPSE_FRAMES as f32;
                    linear.min(1.0)
                }
            }
            _ => 0.0,
        };
        // Ease-in quart: slow start, accelerates
        t * t * t * t
    }

    /// True when we're in the collapse portion of freeze (after flash).
    pub fn is_collapsing(&self) -> bool {
        matches!(self.phase, Phase::Freeze) && !self.is_freeze_flash()
    }
}

fn ramp(frame: u32, start: u32, end: u32) -> f32 {
    if frame < start {
        0.0
    } else if frame >= end {
        1.0
    } else {
        (frame - start) as f32 / (end - start) as f32
    }
}
