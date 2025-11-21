#[derive(Hash, Clone, PartialEq, Eq, Debug)]
pub struct Floor {
    pub side: usize,
    pub active: Box<[u64]>,
}

impl std::fmt::Display for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.side {
            for x in 0..self.side {
                if self.active[y].reverse_bits() & (1 << x) != 0 {
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
        Self {
            side,
            active: vec![0; side].into_boxed_slice(),
        }
    }

    pub fn load(input: &str) -> Self {
        let side = input.lines().next().unwrap().trim().len();
        let mut active = vec![0; side];

        input.lines().enumerate().for_each(|(y, row)| {
            row.bytes()
                .enumerate()
                .filter(|&(_, b)| b == b'#')
                .for_each(|(x, _)| active[y] |= 1 << (63-x));
        });

        Self {
            side,
            active: active.into_boxed_slice(),
        }
    }

    pub fn total_active(&self) -> u32 {
        self.active.iter().map(|row| row.count_ones()).sum()
    }

    // https://www.reddit.com/r/everybodycodes/comments/1p2i1ag/2025_q14_solution_spotlight/npxp3cl/
    pub fn step(&mut self) {
        let mask = !0u64 << (64 - self.side);
        let mut prev_row = 0;
        for y in 0..self.side {
            let next_row = if y < self.side - 1 { self.active[y + 1] } else { 0 };
            let cur_row = &mut self.active[y];
            let new_row = !(prev_row << 1 ^ prev_row >> 1 ^ *cur_row ^ next_row << 1 ^ next_row >> 1) & mask;
            prev_row = *cur_row;
            *cur_row = new_row;
        }
    }
}
