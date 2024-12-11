pub struct Grid {
    inner: Vec<Vec<usize>>,
}

impl Grid {
    pub fn new(s: String) -> Self {
        Self {
            inner: s
                .trim()
                .lines()
                .map(|l| {
                    l.trim()
                        .split("")
                        .filter(|s| *s != "")
                        .map(|c| c.trim().parse::<usize>().unwrap())
                        .collect()
                })
                .collect(),
        }
    }

    pub fn get(&self, row: usize, col: usize) -> usize {
        self.inner[row][col]
    }

    pub fn get_checked(&self, row: usize, col: usize) -> Option<usize> {
        self.inner.get(row)?.get(col).copied()
    }

    pub fn rows(&self) -> usize {
        self.inner.len()
    }

    pub fn cols(&self) -> usize {
        self.inner[0].len()
    }

    /// Return top, left, bottom and right neighbor coordinates of an element.
    pub fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
        if row > 0 {
            neighbors.push((row - 1, col));
        }
        if col + 1 < self.cols() {
            neighbors.push((row, col + 1));
        }
        if row + 1 < self.rows() {
            neighbors.push((row + 1, col));
        }
        if col > 0 {
            neighbors.push((row, col - 1));
        }
        neighbors
    }
}
