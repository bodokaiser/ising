use std::f32;

use rand::Rng;

pub struct Ising {
    beta: f32,
    cols: u32,
    rows: u32,
    data: Vec<i8>,
    energy: i32,
}

impl Ising {
    pub fn new(beta: f32, cols: u32, rows: u32) -> Ising {
        Ising {
            beta: beta,
            cols: cols,
            rows: rows,
            data: vec![-1; (cols * rows) as usize],
            energy: -((cols * rows) as i32),
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

        let delta_energy = 2
            * spin
            * (self.spin(col + 1, row)
                + self.spin(col - 1, row)
                + self.spin(col, row - 1)
                + self.spin(col, row + 1));

        if delta_energy < 0 || rng.gen::<f32>() < (-self.beta * (delta_energy as f32)).exp() {
            self.set_spin(col, row, -spin);

            self.energy += delta_energy as i32;
        }
    }

    pub fn energy(&self) -> i32 {
        return self.energy;
    }

    pub fn magnetisation(&self) -> f64 {
        let sum: i32 = self.data.iter().map(|&spin| spin as i32).sum();

        return f64::from(sum) / (self.data.len() as f64);
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
