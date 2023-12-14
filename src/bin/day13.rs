use aoc2023::{read_lines, run_timed};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum FieldElem {
    Rock,
    Ash,
}

impl FieldElem {
    fn from(c: char) -> FieldElem {
        match c {
            '.' => FieldElem::Ash,
            '#' => FieldElem::Rock,
            _ => panic!("Invalid element {}", c),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum ReflectionLine {
    Col(usize),
    Row(usize),
}

impl ReflectionLine {
    fn get_score(&self) -> usize {
        match self {
            ReflectionLine::Col(n) => *n,
            ReflectionLine::Row(n) => 100 * n,
        }
    }
}

#[derive(Debug, Clone)]
struct Field {
    as_cols: Vec<Vec<FieldElem>>,
    as_rows: Vec<Vec<FieldElem>>,
}

impl Field {
    fn empty() -> Field {
        Field {
            as_cols: Vec::new(),
            as_rows: Vec::new(),
        }
    }

    fn add_row(&mut self, row: &str) {
        let elems: Vec<FieldElem> = row.chars().map(FieldElem::from).collect();

        //Assume input is _square_
        //Fill cols based on first rows length.
        if self.as_rows.len() == 0 {
            while self.as_cols.len() < elems.len() {
                self.as_cols.push(Vec::new());
            }
        }

        if row.len() != self.as_cols.len() {
            panic!("This map aint square!");
        }

        self.as_rows.push(elems.clone());
        for (col, elem) in self.as_cols.iter_mut().zip(elems.into_iter()) {
            col.push(elem)
        }
    }

    fn get_reflection_candidates(&self, required_diff: usize) -> Vec<ReflectionLine> {
        let mut candidates: Vec<ReflectionLine> = Vec::new();
        let check = |in_tuple: &(usize, &[Vec<FieldElem>])| -> bool {
            if let (_, [r1, r2]) = in_tuple {
                let diffsize = count_mismatched_elems(r1, r2);
                diffsize <= required_diff
            } else {
                false
            }
        };
        self.as_rows
            .windows(2)
            .enumerate()
            .filter(check)
            .for_each(|(idx, _)| candidates.push(ReflectionLine::Row(idx + 1)));
        self.as_cols
            .windows(2)
            .enumerate()
            .filter(check)
            .for_each(|(idx, _)| candidates.push(ReflectionLine::Col(idx + 1)));

        candidates
    }

    fn find_reflection_line(&self, required_diff: usize) -> ReflectionLine {
        let candidates = self.get_reflection_candidates(required_diff);

        let mut result = candidates.into_iter().filter(|candidate| {
            let (idx, grid) = match candidate {
                ReflectionLine::Col(i) => (i, &self.as_cols[..]),
                ReflectionLine::Row(i) => (i, &self.as_rows[..]),
            };
            let to_right = (0usize..*idx).rev();
            let to_left = *idx..grid.len();

            let diff: usize = to_right
                .zip(to_left)
                .map(|(l, r)| count_mismatched_elems(&grid[l], &grid[r]))
                .sum();

            diff == required_diff
        });

        if let Some(rl) = result.next() {
            rl
        } else {
            panic!("No reflection in field?!")
        }
    }
}

fn count_mismatched_elems<T: Eq>(l: &[T], r: &[T]) -> usize {
    if l.len() != r.len() {
        panic!("Cannot count mismatched in unequal slices")
    }
    l.iter().zip(r.iter()).filter(|(le, re)| le != re).count()
}

fn read_input() -> Vec<Field> {
    let mut fields = vec![Field::empty()];
    for line in read_lines("./inputs/day13").skip_while(String::is_empty) {
        if line.is_empty() {
            fields.push(Field::empty())
        } else if let Some(fld) = fields.last_mut() {
            fld.add_row(&line)
        }
    }
    fields
}

fn main() {
    let input = read_input();

    println!(
        "Part 1: {}",
        run_timed(|| {
            input
                .iter()
                .map(|f| f.find_reflection_line(0).get_score())
                .sum::<usize>()
        })
    );
    println!(
        "Part 2: {}",
        run_timed(|| {
            input
                .iter()
                .map(|f| f.find_reflection_line(1).get_score())
                .sum::<usize>()
        })
    );
}
