fn get_input_and_parse() -> Vec<Vec<i32>> {
    let mut result :Vec<Vec<i32>> = Vec::new();
    let input = include_str!("../inputs/day2.txt");
    input
        .lines()
        .for_each(|x| {
            result.push(
                x.split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>())});
    result
        
}

fn find_if_safe(x: &Vec<i32>) -> (bool, usize){
    let mut is_safe = true;
    let mut direction = 0;
    let mut had_failed: usize = 0;
    for i in 1..x.len(){
        if ((x[i-1]-x[i]).abs() <1) || ((x[i-1]-x[i]).abs() >3){
            is_safe = false;
            had_failed = i;
            break;
        }
        if direction == 0 {
            if x[i-1]<x[i]{direction = 1;}
            else if x[i-1]>x[i]{direction = 2;}
        }else{
            if direction == 1 && x[i-1]>x[i]{
                is_safe = false;
                had_failed = i;
                break;
            }
            if direction == 2 && x[i-1]<x[i]{
                is_safe = false;
                had_failed = i;
                break;
            }
        }
    }
    (is_safe, had_failed)
}

fn problem1() -> i32{
    let input = get_input_and_parse();
    let mut safe_reports: i32 = 0;
    input.iter().for_each(|x| {
        let (is_safe, _) = find_if_safe(&x);
        if is_safe {
            safe_reports += 1;
        }
    });
    safe_reports
}

fn problem2() -> i32{
    let input = get_input_and_parse();
    let mut safe_reports: i32 = 0;
    input.iter().for_each(|x| {
        let (mut is_safe, failed) = find_if_safe(&x);
        if is_safe{
            safe_reports += 1;
            return;
        }
        //If failed, check with removing the failed element
        let mut new_vec = x.clone();
        new_vec.remove(failed);
        (is_safe, _) = find_if_safe(&new_vec);
        if is_safe{
            safe_reports += 1;
            return;
        }
        //if still failing, check with removing the element before the failed element
        new_vec = x.clone();
        new_vec.remove(failed-1);
        (is_safe, _) = find_if_safe(&new_vec);
        if is_safe{
            safe_reports += 1;
            return;
        }
        //If still failing, check if, for some reason, removing first element makes an report safe
        new_vec = x.clone();
        new_vec.remove(0);
        (is_safe, _) = find_if_safe(&new_vec);

        if is_safe{
            safe_reports += 1;
        }
    });
    safe_reports
}

pub fn day2(){

    println!("Problem 1:{}", problem1());
    println!("Problem 2:{}", problem2());
}