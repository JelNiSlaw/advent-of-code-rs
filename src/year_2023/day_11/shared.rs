pub struct Image {
    grid: Vec<Vec<bool>>,
}

impl Image {
    pub fn parse(lines: Vec<String>) -> Self {
        let grid = lines
            .into_iter()
            .map(|line| line.bytes().map(|byte| byte == b'#').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { grid }
    }

    fn empty_rows(&self) -> Vec<usize> {
        self.grid
            .iter()
            .enumerate()
            .filter(|(_, row)| row.iter().all(|cell| !cell))
            .map(|(i, _)| i)
            .collect::<Vec<_>>()
    }

    fn empty_columns(&self) -> Vec<usize> {
        let mut columns = Vec::new();

        for i in 0..self.grid[0].len() {
            if self.grid.iter().map(|row| row[i]).all(|cell| !cell) {
                columns.push(i);
            }
        }

        columns
    }

    pub fn galaxies(self, expansion_factor: usize) -> Vec<(u64, u64)> {
        let empty_rows = self.empty_rows();
        let empty_columns = self.empty_columns();

        let mut row_offset = 0;

        self.grid
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                if let Some(&empty_row) = empty_rows.get(row_offset) {
                    if empty_row == y {
                        row_offset += 1;
                    }
                }

                let mut column_offset = 0;

                row.into_iter()
                    .enumerate()
                    .filter_map(|(x, cell)| {
                        if let Some(&empty_column) = empty_columns.get(column_offset) {
                            if empty_column == x {
                                column_offset += 1;
                            }
                        }

                        if cell {
                            Some((
                                u64::try_from(x + column_offset * (expansion_factor - 1)).unwrap(),
                                u64::try_from(y + row_offset * (expansion_factor - 1)).unwrap(),
                            ))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}

pub fn distance_sum(galaxies: &[(u64, u64)]) -> u64 {
    (0..galaxies.len())
        .map(|i| {
            let a = galaxies[i];

            (i..galaxies.len())
                .map(|j| {
                    let b = galaxies[j];
                    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
                })
                .sum::<u64>()
        })
        .sum()
}
