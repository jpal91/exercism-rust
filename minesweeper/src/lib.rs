#![allow(unused)]



#[derive(Debug, PartialEq, Clone)]
struct Cell {
    item: u8,
    count: u8,
}

struct Grid {
    g: Vec<Cell>,
    rows: usize,
    cols: usize,
}

struct GridIter {
    g: Vec<Cell>,
    rows: usize,
    cols: usize,
    y: usize,
    x: usize,
}

impl Grid {
    fn from(mf: &[&str]) -> Self {
        let rows = mf.len();
        let cols = mf[0].len();
        let g: Vec<Cell> = mf
            .into_iter()
            .flat_map(|&l| l.as_bytes().to_owned())
            .filter(|&int| int != b'\n')
            .map(|int| Cell{ item: int, count: 0 })
            .collect();

        Self {
            g,
            rows,
            cols,
        }
    }

    fn get<'b>(&'b mut self, row: usize, col: usize) -> &'b mut Cell {
        let coord = (row * self.cols) + col;
        &mut self.g[coord]
    }

    fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let deltas = vec![(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)];
        let max_r = self.rows as isize;
        let max_c = self.cols as isize;
        let mut nbrs: Vec<(usize, usize)> = vec![];

        for (r, c) in deltas {
            let new_row = (row as isize) + r;
            let new_col = (col as isize) + c;

            if (
                new_row < 0 ||
                new_row >= max_r ||
                new_col < 0 ||
                new_col >= max_c
            ) {
                continue
            }

            nbrs.push((new_row as usize, new_col as usize))
        }

        nbrs
    }

    fn to_string_vec(&self) -> Vec<String> {
        let mut it = self.g.iter();
        let mut svec: Vec<String> = vec![];

        for r in 0..self.rows {
            let mut tmp = String::new();
            for c in 0..self.cols {
                let next = it.next().unwrap();

                if next.item == b'*' || next.count == 0 {
                    tmp += std::str::from_utf8(&[next.item]).unwrap()
                } else {
                    tmp += &next.count.to_string()
                }
            }

            svec.push(tmp)
        }

        svec
    }

    fn iter(&self) -> GridIter {

        GridIter {
            g: self.g.clone(),
            rows: self.rows,
            cols: self.cols,
            y: 0,
            x: 0
        }.into_iter()
    }
}

impl Iterator for GridIter {
    type Item = (usize, usize, Cell);

    fn next(&mut self) -> Option<Self::Item> {        
        if self.x >= self.cols {
            self.y += 1;
            self.x = 0;
        }

        if self.y >= self.rows {
            return None
        };

        let coord = (self.y * self.cols) + self.x;
        let (x, y) = (self.x, self.y);

        self.x += 1;

        Some((y, x , self.g[coord].clone()))
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.len() < 1 {
        return vec![]
    }
    
    let mut grid = Grid::from(minefield);
    

    for (y, x, cell) in grid.iter() {
        if cell.item != b'*' {
            continue;
        }

        let neighbors = grid.neighbors(y, x);
        
        for (r ,c) in neighbors {
            let mut cell = grid.get(r, c);
            cell.count += 1;

        }
    }

    grid.to_string_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord() {
        let inp = &[
            "***",
            "*8*",
            "***",
        ];

        let mut grid = Grid::from(inp);

        assert_eq!(grid.get(1, 1), &mut Cell{ item: b'8', count: 0 })
    }
}