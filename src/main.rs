use std::collections::VecDeque;
use std::error::Error;
use std::io;

use std::io::{stdin,stdout,Write};


// return Result !!!!
pub fn get_input() -> (usize, Vec<usize>) {
    let size : usize = read_size();
    let mut vec : Vec<usize> = read_vec();
    // check validity of data !!!
    // ...
    vec.iter_mut().for_each(|mut item| { *item -= 1; });
    (size, vec)
}

fn main()
{
    let (nb_intersec, shortcuts) = get_input();

    //let nb_intersec = 7;
    //let shortcuts = vec![3, 3, 3, 3, 6, 6, 6];

    let mut energy = vec![-1; nb_intersec];

    energy[0] = 0;

    let mut q : VecDeque<usize> = VecDeque::from([0]);

    while !q.is_empty() {

        let idx = q.pop_front().unwrap(); // safe thanks to check is_empty

        // We need to explore all paths

        // both the one that take a lot of energy
        if idx + 1 < nb_intersec && energy[idx + 1] == -1 {
            energy[idx + 1] = energy[idx] + 1;
            q.push_front(idx + 1)
        }

        if idx > 1 && energy[idx - 1] != -1 {
            energy[idx - 1] = energy[idx] - 1;
            q.push_front(idx - 1)
        }


        // and the one with shortcuts (handle shortcuts in priority ie put them on top of the stack)
        if idx < nb_intersec && shortcuts[idx] < nb_intersec && energy[shortcuts[idx]] == -1 {
            energy[shortcuts[idx]] = energy[idx] + 1;
            q.push_front(shortcuts[idx])
        }


        for i in &q {
            print!("{i} ")
        }
        println!("")

    }

    for i in energy {
        print!("{i} ")
    }

}


use std::io::BufRead;

// return Result
fn read_size<T>() -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<T>().unwrap()
}

// return Result
fn read_vec<T>() -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}
