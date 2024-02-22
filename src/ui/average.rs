pub struct Average {
    current_vec: Vec<f64>,
    next_vec: Vec<f64>,
}

impl Average {
    pub fn default() -> Self {
        Self {
            current_vec: vec![0.; 60],
            next_vec: Vec::with_capacity(60),
        }
    }
}

impl Average {
    pub fn update(&mut self, measurement: f64) {
        self.next_vec.push(measurement);
        if self.next_vec.len() >= 60 {
            std::mem::swap(&mut self.current_vec, &mut self.next_vec);
            self.next_vec.clear();
        }
    }

    pub fn get_average(&self) -> f64 {
        let sum: f64 = self.current_vec.iter().sum();
        sum / self.current_vec.len() as f64
    }
}
