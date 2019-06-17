use rand::Rng;

pub struct Ising {
    beta: f64,
    cols: u16,
    rows: u16,
    data: Vec<i8>,
    energy: i32,
    magnetisation: i32,
}

impl Ising {
    pub fn new(temperature: f64, cols: u16, rows: u16) -> Ising {
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

        if energy_diff <= 0 || rng.gen::<f64>() < (-self.beta * f64::from(energy_diff)).exp() {
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

    pub fn absolute_magnetisation(&self) -> f64 {
        let sum: f64 = self
            .data
            .iter()
            .map(|&spin| spin as f64)
            .map(|spin| spin.abs())
            .sum();

        return sum / (self.size() as f64);
    }
}

fn reflect_index(index: i32, boundary: i32) -> u16 {
    if index < 0 {
        return (boundary - (-index % boundary)) as u16;
    }
    if index > boundary - 1 {
        return (index % (boundary - 1)) as u16;
    }

    return index as u16;
}
