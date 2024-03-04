use std::time::Duration;

use crate::gameboy::{Gameboy, CLOCK_SPEED_HZ};
use crate::time::Instant;

pub struct Governor {
    speed: f64,
    first_time: Option<Instant>,
    cycles: u128,
    true_cycles: u128,
    last_time: Option<Instant>,
    last_cycles: Option<u128>,
    real_speed: f64,
    edit_string: String,
}

impl Default for Governor {
    fn default() -> Self {
        let speed = 1.;
        Self {
            edit_string: speed.to_string(),
            speed,
            first_time: None,
            cycles: 0,
            true_cycles: 0,
            last_time: None,
            last_cycles: None,
            real_speed: 0.,
        }
    }
}

impl Governor {
    fn cycles_to_run(&self, now: &Instant) -> u128 {
        let run_time = now.duration_since(self.first_time.unwrap());
        let true_hz = (self.speed * CLOCK_SPEED_HZ as f64) as u128;
        let target_cycles = run_time.as_micros() * true_hz / 1_000_000;
        target_cycles - self.cycles
    }

    pub fn tick(&mut self, gameboy: &mut Gameboy, console: &mut String) {
        if self.first_time.is_none() {
            self.first_time = Some(Instant::now());
        }

        let now = Instant::now();

        let cycles = self.cycles_to_run(&now);
        for _ in 0..cycles {
            if let Some(c) = gameboy.tick() {
                console.push(c as char);
            }
        }
        self.cycles += cycles;
        self.true_cycles += cycles;

        if let Some(last_time) = self.last_time {
            let duration = now.duration_since(last_time);

            if duration >= Duration::from_secs(1) {
                let last_cycles = self.last_cycles.unwrap();
                let cycles = self.true_cycles - last_cycles;
                self.real_speed = (cycles as f64) / (CLOCK_SPEED_HZ as f64);

                self.last_time = Some(now);
                self.last_cycles = Some(self.cycles);
                self.true_cycles = self.cycles;
            }
        } else {
            self.last_time = Some(Instant::now());
            self.last_cycles = Some(self.cycles);
        }
    }

    pub fn skip(&mut self) {
        let now = Instant::now();
        self.cycles += self.cycles_to_run(&now)
    }

    pub fn average_speed(&self) -> f64 {
        self.real_speed
    }

    pub fn set_speed(&mut self, speed: f64) {
        let mut gov = Self {
            edit_string: speed.to_string(),
            speed,
            ..Default::default()
        };

        std::mem::swap(self, &mut gov);
    }

    pub fn edit_string(&mut self) -> &mut String {
        &mut self.edit_string
    }
}
