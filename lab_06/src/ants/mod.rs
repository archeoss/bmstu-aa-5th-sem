pub mod ant_solver;
pub mod brute_solver;
pub mod constants;
pub mod utils;

use std::time::Instant;

use ant_solver::AntSolver;
use brute_solver::BruteSolver;
use utils::{generate_data, print_data, read_config, read_data};

pub type Cost = usize;

pub fn run_time()
{
    let mut config = read_config(constants::CONFIG_FILE);
    for s in constants::TIME_FROM..=constants::TIME_TO {
        print!("Size: {s}, ");
        let data = generate_data(
            s,
            constants::VALS_FROM,
            constants::VALS_TO,
            constants::RIVER_FROM,
            constants::RIVER_TO,
        );
        config.m = data.len();
        config.start = 0;
        config.end = config.m - 1;
        let time = Instant::now();
        let brute = BruteSolver::new(&data, config.start, config.end);
        let (lb, tb) = brute.solve();
        print!("BruteTime: {} mcs, ", time.elapsed().as_nanos());

        let time = Instant::now();
        let ant = AntSolver::new(&data, config.clone());
        let (la, ta) = ant.solve();
        // assert_eq!(la, lb, "{data:?}\n{ta:?}\n{tb:?}");
        println!("AntTime: {} mcs;", time.elapsed().as_nanos());
        if lb == usize::MAX {
            println!("Destination is unaccessible");
        }
    }
}

pub fn run_parametrization()
{
    let mut conf = read_config(constants::CONFIG_FILE);
    conf.m = constants::TEST_SIZE;
    conf.start = 0;
    conf.end = conf.m - 1;
    for (from, to, river_from, river_to) in &[
        (
            constants::VALS_FROM,
            constants::VALS_TO,
            constants::RIVER_FROM,
            constants::RIVER_TO,
        ),
        (
            constants::BIG_VALS_FROM,
            constants::BIG_VALS_TO,
            constants::BIG_RIVER_FROM,
            constants::BIG_RIVER_TO,
        ),
    ] {
        let data = generate_data(constants::TEST_SIZE, *from, *to, *river_from, *river_to);
        print_data(&data);
        let brute = BruteSolver::new(&data, 0, constants::TEST_SIZE - 1);
        let (best_l, _) = brute.solve();
        if best_l == usize::MAX {
            println!("Destination is unaccessible\n");
        } else {
            println!("Best Distance: {best_l}\n");
        }
        if best_l != usize::MAX {
            conf.alpha = constants::ALPHA_START;
            while conf.alpha < constants::ALPHA_END + constants::ALPHA_STEP / 2_f64 {
                conf.beta =
                    f64::round((1.0 - conf.alpha) * constants::PRESISION) / constants::PRESISION;
                conf.p = constants::P_START;
                while conf.p < constants::P_END + constants::P_STEP / 2_f64 {
                    let ant_solver = AntSolver::new(&data, conf.clone());
                    let (len, rt) = ant_solver.solve();
                    println!(
                        "{:<4} & {:<4} & {:<4} & {:<5} & {:<5} & {rt:?}\\\\",
                        conf.alpha,
                        conf.beta,
                        conf.p,
                        len,
                        len - best_l
                    );
                    conf.p = f64::round((conf.p + constants::P_STEP) * constants::PRESISION)
                        / constants::PRESISION;
                }
                conf.alpha =
                    f64::round((conf.alpha + constants::ALPHA_STEP) * constants::PRESISION)
                        / constants::PRESISION;
            }
        }
    }
}

pub fn run_interactive()
{
    let data = read_data(constants::DATA_FILE);
    let config = read_config(constants::CONFIG_FILE);
    println!("Data:");
    print_data(&data);
    println!("\n");

    let brute = BruteSolver::new(&data, config.start, config.end);
    let (len, path) = brute.solve();
    if len == usize::MAX {
        println!("Destination is unaccessible\n");
    } else {
        println!("BRUTE:\nLength: {len:>5}, Path: {path:?}");
    }
    let ant_solver = AntSolver::new(&data, config);
    let (len, path) = ant_solver.solve();
    if len == usize::MAX {
        println!("Destination is unaccessible\n");
    } else {
        println!("ANTS:\nLength: {len:>5}, Path: {path:?}");
    }
}
