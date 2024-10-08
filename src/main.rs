use num_bigint::BigInt;
use rayon::prelude::*;
use std::time::SystemTime;

fn main() {
    let res_x = "5547094896230060345977898543873469282119259956812769264843946971664050560756"
        .parse::<BigInt>()
        .unwrap();
    let res_y = "14961832535963026880436662513768132861653428490706468784706450723166120307238"
        .parse::<BigInt>()
        .unwrap();
    let curve_gen_x =
        "14810849444223915365675197147935386463496555902363368947484943637353816116538"
            .parse::<BigInt>()
            .unwrap();
    let curve_gen_y = "742647408428947575362456675910688304313089065515277648070767281175728054553"
        .parse::<BigInt>()
        .unwrap();
    let p = "21888242871839275222246405745257275088696311157297823662689037894645226208583"
        .parse::<BigInt>()
        .unwrap();
    println!("res_x: {:#?}", res_x);
    println!("res_y: {:#?}", res_y);
    println!("curve_gen_x: {:#?}", curve_gen_x);
    println!("curve_gen_y: {:#?}", curve_gen_y);
    println!("p: {:#?}", p);

    // let x = 0..i32::MAX;
    // let x: Vec<i128> =
    //     ((i32::MAX as i128 + 1_000_000_000)..(i32::MAX as i128 + 10_000_000_000)).collect();
    let x: Vec<i32> = (0..i32::MAX).collect();
    // println!("i32::MAX: {:?}", i32::MAX);
    // println!("{:?}", i32::MAX as i64);
    // println!("{:#?}", x);

    let start = SystemTime::now();

    // let filtered = x
    //     .into_par_iter()
    //     .filter(|&num| res_x == (num * &curve_gen_x) % &p)
    //     .collect::<Vec<_>>();

    let filtered = x
        .par_iter()
        .find_any(|&num| res_x == (num * &curve_gen_x) % &p)
        .cloned()
        .unwrap();

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("it took {} seconds", duration.as_secs());
    println!("{:?}", filtered);

    // (x, y) is on the curve
    // let lhs = (&res_y * &res_y) % &p;
    // println!("lhs: {:#?}", lhs);
    // let rhs = (&res_x * &res_x * &res_x + 2023) % &p;
    // println!("lhs: {:#?}", lhs);
    // assert_eq!(lhs, rhs);
}
