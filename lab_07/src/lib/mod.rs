use csv::ReaderBuilder;
use num_format::{Locale, ToFormattedString};
use std::cmp::Ord;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

use colored::Colorize;
mod maps;
mod terms;
pub use maps::{BruteMap, DictEntry, Map, Moa, Name};
mod constants;
pub use terms::generate_job;

use crate::lib::terms::{Task, Terms};
fn read_data(filename: &str) -> Vec<DictEntry<Name, Moa>> {
    let mut file = File::open(filename).unwrap();
    let mut res = Vec::new();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(data.as_bytes());
    for elem in reader.deserialize::<DictEntry<_, _>>() {
        res.push(elem.unwrap());
    }
    res
}

pub fn run_tests() {
    let source = read_data(constants::DATA_PATH);
    let brutemap = BruteMap::from(source.clone());
    // let binmap = BinaryMap::from(source.clone());

    println!("================== BruteMap  ========================");
    measure(
        brutemap,
        &source,
        (&constants::GOOD_KEYS_AMOUNTS, &constants::BAD_KEYS_AMOUNTS),
    );
    println!("=====================================================");
}

fn measure<T: Map<U, V>, U: Clone + Ord, V: PartialOrd + Copy>(
    map: T,
    source: &[DictEntry<U, V>],
    (gks, bks): (&[usize], &[usize]),
) {
    let slen = source.len();
    for &ka in gks {
        let gktime = Instant::now();
        for key in (0..ka).map(|i| source[i % slen].key.clone()) {
            map.get(&key);
        }
        let gktime = gktime.elapsed().as_nanos();
        println!(
            "GOOD_KEYS:\namount={}\ntime={}\n",
            ka.to_formatted_string(&Locale::en),
            gktime.to_formatted_string(&Locale::en),
        );
    }

    println!("");

    for &ka in bks {
        let bktime = Instant::now();
        for key in (0..ka).map(|i| source[i % slen].key.clone()) {
            map.get(&key);
        }
        let bktime = bktime.elapsed().as_nanos();
        println!(
            "BAD_KEYS:\namount={}\ntime={}\n",
            ka.to_formatted_string(&Locale::en),
            bktime.to_formatted_string(&Locale::en)
        );
    }
}

pub fn run_interactive() {
    let source = read_data(constants::DATA_PATH);
    let brutemap = BruteMap::from(source.clone());

    loop {
        println!("Please enter a string:");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let job = generate_job(input.as_mut_str());
        if let Some(mut job) = job {
            println!(
                "{}",
                "Будут выведены следующие категории стрелков:".yellow()
            );
            match job.task {
                Task::LikeSomebody => {
                    for shooter in &job.shooters {
                        let person = brutemap.get(&shooter.trim().to_string());
                        if let Some(moa) = person {
                            let it = Terms::iter();
                            for term in it {
                                if moa >= term.value().0
                                    && moa < term.value().1
                                    && !job.except.contains(term)
                                {
                                    job.terms.push(*term);
                                    println!(
                                        "{} на промежутке [{}, {})",
                                        term.to_string().red(),
                                        term.value().0,
                                        term.value().1
                                    );
                                    break;
                                }
                            }
                        }
                    }
                }
                Task::AllExcept | Task::ExactTerm => {
                    for term in &job.terms {
                        println!(
                            "{} на промежутке [{}, {})",
                            term.to_string().red(),
                            term.value().0,
                            term.value().1
                        );
                    }
                }
                Task::InBetween => {
                    if job.terms[0].value().1 > job.terms[1].value().0 {
                        job.terms.swap(0, 1);
                    }
                    let mut it = Terms::iter();
                    while it.next().unwrap().clone() != job.terms[0] {}
                    println!(
                        "{} на промежутке [{}, {})",
                        job.terms[0].to_string().red(),
                        job.terms[0].value().0,
                        job.terms[0].value().1
                    );

                    let mut term = it.next().unwrap().clone();
                    while term != job.terms[1] {
                        if !job.except.contains(&term) {
                            println!(
                                "{} на промежутке [{}, {})",
                                term.to_string().red(),
                                term.value().0,
                                term.value().1
                            );
                            job.terms.push(term);
                        }
                        term = it.next().unwrap().clone();
                    }
                    println!(
                        "{} на промежутке [{}, {})",
                        job.terms[1].to_string().red(),
                        job.terms[1].value().0,
                        job.terms[1].value().1
                    );
                }
                Task::WorseThan => {
                    let mut it = Terms::iter();
                    while it.next().unwrap().clone() != job.terms[0] {}
                    println!(
                        "{} на промежутке [{}, {})",
                        job.terms[0].to_string().red(),
                        job.terms[0].value().0,
                        job.terms[0].value().1
                    );

                    for term in it {
                        if job.except.contains(term) {
                            continue;
                        }
                        println!(
                            "{} на промежутке [{}, {})",
                            term.to_string().red(),
                            term.value().0,
                            term.value().1
                        );
                        job.terms.push(term.clone());
                    }
                }
                Task::BetterThan => {
                    let it = Terms::iter();

                    for term in it {
                        if job.except.contains(term) {
                            continue;
                        }
                        println!(
                            "{} на промежутке [{}, {})",
                            term.to_string().red(),
                            term.value().0,
                            term.value().1
                        );
                        if job.terms[0] == *term {
                            break;
                        }
                        job.terms.push(term.clone());
                    }
                }
            }
            if job.terms.is_empty() {
                println!("Не удалось найти подходящие термы.");
                continue;
            }
            let mut res = vec![];
            for term in &job.terms {
                let res_temp = brutemap.search(term.value());
                res.extend(res_temp.iter().cloned());
            }
            for entry in res {
                println!(
                    "{} --- {} MOA",
                    entry.key.green(),
                    entry.value.to_string().red()
                );
            }
        } else {
            println!("\nВведена неверная строка, повторите запрос.\n");
        }
    }
}
