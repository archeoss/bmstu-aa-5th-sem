use super::Cost;
use itertools::Itertools;

pub struct BruteSolver<'a>
{
    data: &'a [Vec<Cost>],
    start: usize,
    end: usize,
}

impl<'a> BruteSolver<'a>
{
    pub const fn new(data: &'a [Vec<Cost>], start: usize, end: usize) -> Self
    {
        Self { data, start, end }
    }

    pub fn solve(&self) -> (Cost, Vec<usize>)
    {
        let (mut best_l, mut best_t) = (Cost::MAX, Vec::new());

        let left = vec![
            (0..self.start).collect_vec(),
            ((self.start + 1)..self.end).collect_vec(),
            ((self.end + 1)..self.data.len()).collect_vec(),
        ]
        .concat();
        for length in 0..=(self.data.len() - 2) {
            for permutation in left.iter().permutations(length) {
                let route = vec![
                    vec![self.start],
                    permutation.into_iter().copied().collect(),
                    vec![self.end],
                ]
                .concat();
                let l = self.compute_dist(&route);
                if l < best_l {
                    best_l = l;
                    best_t = route;
                }
            }
        }
        (best_l, best_t)
    }

    fn compute_dist(&self, t: &[usize]) -> Cost
    {
        match t.len() {
            0..=1 => 0 as Cost,
            2 => self.data[t[0]][t[1]],
            _ => t.windows(2).fold(0 as Cost, |acc, el| {
                acc.saturating_add(self.data[el[0]][el[1]])
            }),
        }
    }
}
