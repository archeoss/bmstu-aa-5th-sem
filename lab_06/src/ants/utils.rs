use super::ant_solver::Config;
use super::{constants, Cost};
use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn read_data(filename: &str) -> Vec<Vec<Cost>>
{
    let mut f = File::open(filename).unwrap();
    let mut res = String::new();
    f.read_to_string(&mut res).unwrap();
    let mut data_temp: Vec<Vec<isize>> = serde_json::from_str(&res).unwrap();
    data_temp
        .iter_mut()
        .map(|vec| {
            vec.iter()
                .map(|&elem| elem as usize)
                .collect::<Vec<usize>>()
        })
        .collect()
}

pub fn generate_data(
    size: usize,
    from: Cost,
    to: Cost,
    from_river: Cost,
    to_river: Cost,
) -> Vec<Vec<Cost>>
{
    let mut rng = thread_rng();
    let mut data = vec![vec![0 as Cost; size]; size];
    for i in 0..size {
        for j in (i + 1)..size {
            if rng.gen_range(0.0f64..1.0f64) > constants::BLOCK_PERC {
                let val = rng.gen_range(from..to);
                data[i][j] = val;
                data[j][i] = val + rng.gen_range(from_river..to_river);
            } else {
                data[i][j] = usize::MAX;
                data[j][i] = usize::MAX;
            }
        }
    }
    data
}

pub fn read_config(filename: &str) -> Config
{
    let mut f = File::open(filename).unwrap();
    let mut res = String::new();
    f.read_to_string(&mut res).unwrap();
    serde_json::from_str(&res).unwrap()
}

pub fn print_data(data: &[Vec<Cost>])
{
    for row in data {
        print!("{:>5} ", row[0] as isize);
        row.iter()
            .skip(1)
            .for_each(|&e| print!("& {:>5} ", e as isize));
        println!("\\\\");
    }
}
