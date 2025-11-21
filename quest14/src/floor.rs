#[derive(Hash, Clone, PartialEq, Eq, Debug)]
pub struct Floor {
    pub side: usize,
    pub active: Vec<u64>,
}

impl std::fmt::Display for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.side {
            for x in 0..self.side {
                if self.active[y] & (1 << x) != 0 {
                    write!(f, "█")?;
                } else {
                    write!(f, "░")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Floor {
    pub fn empty(side: usize) -> Self {
        Self {side, active: vec![0; side]}
    }

    pub fn load(input: &str) -> Self {
        let side = input.lines().next().unwrap().trim().len();
        let mut active = vec![0; side];

        input.lines().enumerate().for_each(|(y, row)| {
            row.bytes()
                .enumerate()
                .filter(|&(_, b)| b == b'#')
                .for_each(|(x, _)| active[y] |= 1 << x);
        });

        Self { side, active }
    }

    pub fn total_active(&self) -> u32 {
        self.active.iter().map(|row| row.count_ones()).sum()
    }

    pub fn step(&mut self) {
        let mut prev_row = 0;
        for y in 0..self.side {
            let next_row = if y < self.side - 1 { self.active[y + 1] } else { 0 };
            let cur_row = &mut self.active[y];
            let mut new_row = 0;
            for x in 0..self.side {
                let count = [
                    x != 0 && prev_row & (1 << (x - 1)) != 0,
                    prev_row & (1 << (x + 1)) != 0,
                    x != 0 && next_row & (1 << (x - 1)) != 0,
                    next_row & (1 << (x + 1)) != 0,
                ]
                .into_iter()
                .filter(|&x| x)
                .count();
                let cur_active = *cur_row & (1 << x) != 0;
                if (cur_active && count % 2 != 0) || (!cur_active && count % 2 == 0) {
                    new_row |= 1 << x;
                }
            }
            prev_row = *cur_row;
            *cur_row = new_row;
        }
    }
}
