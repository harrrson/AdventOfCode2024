fn get_input() -> (Vec<i32>, Vec<i32>) {
    let input = include_str!("../inputs/day1.txt");
    let (mut a, mut b) = (Vec::new(), Vec::new());
    input
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .for_each(|x| {
            a.push(x[0].parse::<i32>().unwrap());
            b.push(x[1].parse::<i32>().unwrap());
        });
    (a, b)
        
}

fn problem1() {
    let (mut a, mut b) = get_input();
    a.sort();
    b.sort();
    let mut sum = 0;
    for i in 0..a.len() {
        sum += (a[i] - b[i]).abs();
    }
    println!("Problem 1: {}", sum);
}

use std::collections::HashMap;

fn problem2() {
    let (a, b) = get_input();
    let mut sum = 0;
    let mut b_map: HashMap<i32,i32> = HashMap::new();
    b.into_iter().for_each(|x| {
        if b_map.contains_key(&x) {
            *b_map.get_mut(&x).unwrap() += 1;
        } else {
            b_map.insert(x, 1);
        }
    });

    a.into_iter().for_each(|x| {
        if b_map.contains_key(&x) {
            sum += x * (*(b_map.get_mut(&x).unwrap()));
        }
    });
    println!("Problem 2: {}", sum);
}

pub fn day1() {
    problem1();
    problem2();
}