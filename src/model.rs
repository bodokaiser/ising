use std::f32;

use rand::Rng;

pub struct Ising {
    beta: f32,
    cols: u32,
    rows: u32,
    data: Vec<i8>,
    energy: i32,
    magnetisation: i32,
}

impl Ising {
    pub fn new(temperature: f32, cols: u32, rows: u32) -> Ising {
        let size = (cols * rows) as i32;

        Ising {
            beta: 1.0 / temperature,
            cols: cols,
            rows: rows,
            data: vec![1; size as usize],
            energy: -2 * size,
            magnetisation: size,
        }
    }

    pub fn spin(&self, i: i32, j: i32) -> i8 {
        let col = reflect_index(i, self.cols as i32);
        let row = reflect_index(j, self.rows as i32);

        return self.data[(col * self.cols + row) as usize];
    }

    pub fn set_spin(&mut self, i: i32, j: i32, value: i8) {
        let col = reflect_index(i, self.cols as i32);
        let row = reflect_index(j, self.rows as i32);

        self.data[(col * self.cols + row) as usize] = value;
    }

    pub fn step(&mut self) {
        let mut rng = rand::thread_rng();

        let col = rng.gen_range(0, self.cols) as i32;
        let row = rng.gen_range(0, self.rows) as i32;

        let spin = self.spin(col, row);

        let energy_diff = 2
            * spin
            * (self.spin(col + 1, row)
                + self.spin(col - 1, row)
                + self.spin(col, row - 1)
                + self.spin(col, row + 1));

        let epsilon = rng.gen::<f32>();

        if energy_diff <= 0 || epsilon < (-self.beta * f32::from(energy_diff)).exp() {
            self.set_spin(col, row, -spin);

            self.energy += energy_diff as i32;
            self.magnetisation -= 2 * spin as i32;
        }
    }

    pub fn size(&self) -> usize {
        return self.data.len();
    }

    pub fn energy(&self) -> i32 {
        return self.energy;
    }

    pub fn magnetisation(&self) -> i32 {
        return self.magnetisation;
    }

    pub fn susceptibility(&self) -> f64 {
        let mean = f64::from(self.magnetisation()) / (self.size() as f64);

        let sum: f64 = self
            .data
            .iter()
            .map(|&spin| spin as f64)
            .map(|spin| (spin - mean).powf(2.0))
            .sum();

        return (self.beta as f64) * sum / (self.size() as f64);
    }
}

fn reflect_index(index: i32, boundary: i32) -> u32 {
    if index < 0 {
        return (boundary - (-index % boundary)) as u32;
    }
    if index > boundary - 1 {
        return (index % (boundary - 1)) as u32;
    }

    return index as u32;
}
