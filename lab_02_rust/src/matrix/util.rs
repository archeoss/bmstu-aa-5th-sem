use std::io::{Write};
use colored::Colorize;
use rand::Rng;

pub fn input_matrix(rows: usize, cols: usize) -> Vec<Vec<i32>> {
    let mut matrix = Vec::new();
    for i in 0..rows {
        let mut row = String::new();
        print!("row {}: ", i + 1);
        std::io::stdout().flush().expect("");
        std::io::stdin().read_line(&mut row).expect("Can't read a line!");

        let row: Vec<_> = row.split_ascii_whitespace()
            .map(|elem| elem.parse::<i32>().expect("Input Error, not a number!"))
            .collect();

        if row.len() != cols {
            panic!("Введено неверное кол-во элементов для столбца!");
        } else {
            matrix.push(row);
        }
    }

    matrix
}

pub fn read_data() -> (Vec<Vec<i32>>, Vec<Vec<i32>>)
{
    print!("{}", "Введите a, b, c через пробел (матрицы - a x b, b x c): ".to_string().green());
    std::io::stdout().flush().expect("Can't print!");
    let mut sizes = String::new();
    std::io::stdin().read_line(&mut sizes).expect("Can't read sizes.");

    let sizes: Vec<_> = sizes.split_ascii_whitespace()
        .map(|elem| elem.parse::<usize>().expect("Can't convert into integer."))
        .collect();
    let sizes = if sizes.len() >= 3 {
        [sizes[0], sizes[1], sizes[2]]
    } else {
        panic!("Input error!")
    };
    println!("{}",format!("Input matrix A : [{} x {}]", sizes[0], sizes[1]).to_string().yellow());
    let m1 = input_matrix(sizes[0], sizes[1]);
    println!("{}",format!("Input matrixB : [{} x {}]", sizes[1], sizes[2]).to_string().yellow());
    let m2 = input_matrix(sizes[1], sizes[2]);

    (m1, m2)
}

pub trait PrintMatrix {
    fn print(&self);
}

impl PrintMatrix for Vec<Vec<i32>> {
    fn print(&self) {
        for row in self {
            for elem in row {
                print!("{} ", elem);
            }
            println!();
        }
    }
}

pub fn generate_matrix(rows : usize, cols : usize) -> Vec<Vec<i32>>
{
    let mut rng = rand::thread_rng();
    (0..rows).map(|_| (
        (0..cols).map(|_|
            rng.gen::<i32>() % 100)
                .collect::<Vec<i32>>()))
                    .collect()
}