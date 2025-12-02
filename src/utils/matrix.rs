use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};

#[derive(Default, Clone)]
pub struct Matrix<T> {
    width: usize,
    height: usize,
    data: Vec<Vec<T>>,
}

#[allow(dead_code)]
impl<T> Matrix<T>
where
    T: Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![vec![T::default(); width]; height],
        }
    }

    pub fn map<T2>(&self, f: impl Fn(&T) -> T2) -> Matrix<T2> {
        let mut rows: Vec<Vec<T2>> = Vec::new();
        for y in 0..self.height {
            let mut columns: Vec<T2> = Vec::new();
            for x in 0..self.width {
                let val = self.get(x, y).unwrap();
                columns.push(f(val));
            }
            rows.push(columns);
        }

        Matrix {
            width: rows[0].len(),
            height: rows.len(),
            data: rows,
        }
    }

    pub fn map_xy<T2>(&self, mut f: impl FnMut(&T, usize, usize) -> T2) -> Matrix<T2> {
        let mut rows: Vec<Vec<T2>> = Vec::new();
        for y in 0..self.height {
            let mut columns: Vec<T2> = Vec::new();
            for x in 0..self.width {
                let val = self.get(x, y).unwrap();
                columns.push(f(val, x, y));
            }
            rows.push(columns);
        }

        Matrix {
            width: rows[0].len(),
            height: rows.len(),
            data: rows,
        }
    }

    pub fn count<F>(&self, f: F) -> usize
    where
        F: Fn(&T) -> bool,
    {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let val = self.get(x, y).unwrap();
                if f(val) {
                    count += 1;
                }
            }
        }

        count
    }
}

impl<T> Matrix<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y).and_then(|row| row.get(x))
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.data.get_mut(y).and_then(|row| row.get_mut(x))
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.data[y][x] = value;
    }
}

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn new_fill(width: usize, height: usize, fill: T) -> Self {
        Self {
            width,
            height,
            data: vec![vec![fill; width]; height],
        }
    }
}

#[allow(dead_code)]
impl<T> Matrix<T>
where
    T: Default + Clone + Eq + PartialEq,
{
    pub fn flood_eq<F>(&self, x: usize, y: usize, mut f: F) -> Matrix<bool>
    where
        F: FnMut(&usize, &usize),
    {
        let mut visit_mat: Matrix<bool> = Matrix::new(self.width, self.height);
        let val = self.get(x, y).unwrap();
        self.flood_eq_impl(x, y, val, &mut f, &mut visit_mat);

        visit_mat
    }

    fn flood_eq_impl<F>(&self, x: usize, y: usize, value: &T, f: &mut F, mut visit_mat: &mut Matrix<bool>)
    where
        F: FnMut(&usize, &usize),
    {
        if let Some(visited) = visit_mat.get(x, y) {
            if *visited {
                return;
            }
        } else {
            return;
        }

        if *self.get(x, y).unwrap() != *value {
            return;
        }

        f(&x, &y);
        visit_mat.set(x, y, true);

        if x > 0 {
            self.flood_eq_impl(x - 1, y, value, f, &mut visit_mat);
        }
        if y > 0 {
            self.flood_eq_impl(x, y - 1, value, f, &mut visit_mat);
        }
        if x < self.width - 1 {
            self.flood_eq_impl(x + 1, y, value, f, &mut visit_mat);
        }
        if y < self.height - 1 {
            self.flood_eq_impl(x, y + 1, value, f, &mut visit_mat);
        }
    }

    pub fn flood_where<F>(&self, x: usize, y: usize, mut f: F) -> Matrix<bool>
    where
        F: FnMut(&usize, &usize, &T) -> bool,
    {
        let mut visit_mat: Matrix<bool> = Matrix::new(self.width, self.height);
        let val = self.get(x, y).unwrap();
        self.flood_where_impl(x, y, val, &mut f, &mut visit_mat);

        visit_mat
    }

    fn flood_where_impl<F>(&self, x: usize, y: usize, value: &T, f: &mut F, mut visit_mat: &mut Matrix<bool>)
    where
        F: FnMut(&usize, &usize, &T) -> bool,
    {
        if let Some(visited) = visit_mat.get(x, y) {
            if *visited {
                return;
            }
        } else {
            return;
        }

        if *self.get(x, y).unwrap() != *value {
            return;
        }

        let value = self.get(x, y).unwrap();
        if !f(&x, &y, value) {
            return;
        }
        visit_mat.set(x, y, true);

        if x > 0 {
            self.flood_where_impl(x - 1, y, value, f, &mut visit_mat);
        }
        if y > 0 {
            self.flood_where_impl(x, y - 1, value, f, &mut visit_mat);
        }
        if x < self.width - 1 {
            self.flood_where_impl(x + 1, y, value, f, &mut visit_mat);
        }
        if y < self.height - 1 {
            self.flood_where_impl(x, y + 1, value, f, &mut visit_mat);
        }
    }

    pub fn each<F>(&self, mut f: F)
    where
        F: FnMut(&usize, &usize, &T),
    {
        for y in 0..self.height {
            for x in 0..self.width {
                f(&x, &y, &self.get(x, y).unwrap());
            }
        }
    }

    pub fn positions<F>(&self, mut f: F) -> HashSet<(usize, usize)>
    where
        F: FnMut(&usize, &usize, &T) -> bool,
    {
        let mut positions = HashSet::new();

        for x in 0..self.width {
            for y in 0..self.height {
                let val = self.get(x, y).unwrap();
                if f(&x, &y, &val) {
                    positions.insert((x, y));
                }
            }
        }

        positions
    }
}

impl<T> Matrix<T>
where
    T: Default + Clone + From<char>,
{
    pub fn from_text(text: &str) -> Self {
        let mut rows: Vec<Vec<T>> = Vec::new();
        for line in text.lines() {
            let mut columns: Vec<T> = Vec::new();
            for ch in line.chars() {
                columns.push(ch.into());
            }
            rows.push(columns);
        }

        Self {
            width: rows[0].len(),
            height: rows.len(),
            data: rows,
        }
    }
}

impl<T> Debug for Matrix<T>
where
    T: Debug + Clone + Default + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut data_hash_map: HashMap<(usize, usize), String> = HashMap::new();
        let mut max_string_length = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.get(x, y).unwrap();
                let cell_string = format!("{}", cell);
                max_string_length = max_string_length.max(cell_string.len());
                data_hash_map.insert((x, y), cell_string);
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let string = data_hash_map.get(&(x, y)).unwrap();
                write!(f, "{:width$}", string, width = max_string_length + 1)?;
            }
            if y < self.height - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}