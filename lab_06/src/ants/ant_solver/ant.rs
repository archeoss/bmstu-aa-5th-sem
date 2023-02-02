use super::Cost;
use rand::prelude::*;

#[derive(Clone)]
pub struct Ant
{
    pub route: Vec<usize>,
    len: Cost,
    left: Vec<usize>,
    dest: usize,
}

impl Ant
{
    pub fn new(cities_amount: usize, start: usize, dest: usize) -> Self
    {
        let left: Vec<usize> = (0..cities_amount).filter(|&e| e != start).collect();
        Self {
            route: vec![start],
            len: 0 as Cost,
            left,
            dest,
        }
    }

    pub fn walk(
        &mut self,
        d: &[Vec<Cost>],
        nd: &[Vec<f64>],
        pd: &[Vec<f64>],
        alpha: f64,
        beta: f64,
        rng: &mut ThreadRng,
    )
    {
        for _ in 0..self.left.len() {
            self.next(d, nd, pd, alpha, beta, rng);
            if self.route[self.route.len() - 1] == self.dest {
                break;
            }
        }
    }

    pub fn data(&self) -> (Cost, &[usize])
    {
        (self.len, &self.route)
    }

    fn next(
        &mut self,
        d: &[Vec<Cost>],
        nd: &[Vec<f64>],
        pd: &[Vec<f64>],
        alpha: f64,
        beta: f64,
        rng: &mut ThreadRng,
    )
    {
        let cur = self.route[self.route.len() - 1];
        let denominator = self.left.iter().fold(0.0, |acc, &e| {
            f64::powf(pd[cur][e], alpha).mul_add(f64::powf(nd[cur][e], beta), acc)
        });
        let mut pick: f64 = rng.gen();
        for (index, &j) in self.left.iter().enumerate() {
            let cur_prob = f64::powf(pd[cur][j], alpha) * f64::powf(nd[cur][j], beta);
            pick -= cur_prob / denominator;
            if pick < 0_f64 {
                self.pick(index, d[cur][j]);
                return;
            }
        }
        let index = self.left.len() - 1;
        self.pick(index, d[cur][self.left[index]]);
    }

    fn pick(&mut self, index: usize, diff: Cost)
    {
        self.route.push(self.left.remove(index));
        self.len = self.len.saturating_add(diff);
    }
}
