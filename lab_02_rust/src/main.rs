use colored::Colorize;
// use util::{read_data, PrintMatrix};
// use algos::*;
use lab_02::matrix::util::{read_data, PrintMatrix};
use lab_02::matrix::algos::*;
fn main() {
    let (m1,m2) = read_data();
    // m1.print();
    // m2.print();

    let naive = m1.naive_mut(&m2);
    let winograd = m1.winograd_mut(&m2);
    let winograd_imp = m1.winograd_mut_improved(&m2);

    println!("{}","Multiplication results.".to_string().yellow());
    println!("{}","Naive multiplication:".to_string().purple());
    naive.print();

    println!("{}", "Winograd multiplication:".to_string().purple());
    winograd.print();

    println!("{}", "Winograd multiplication improved:".to_string().purple());
    winograd_imp.print();
}