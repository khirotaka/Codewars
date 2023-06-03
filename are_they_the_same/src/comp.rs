use std::collections::{HashMap, HashSet};

fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut result: Vec<bool> = vec![];
    let mut a_vs_b_dict: HashMap<i64, u16> = HashMap::new();
    let mut b_vs_a_dict: HashMap<i64, u16> = HashMap::new();

    for b_i in &b {
        let mut tmp: Vec<bool> = vec![];
        let a_set: HashSet<i64> = a.clone().into_iter().collect();
        for a_i in &a_set {
            let a_i_key = a_i.abs();
            let n_a = match a_vs_b_dict.get(&a_i_key) {
                Some(v) => *v,
                None => 0,
            };

            if a_i.pow(2) == *b_i {
                a_vs_b_dict.insert(a_i_key, n_a + 1);
                tmp.push(true);
            }
            else {
                tmp.push(false)
            }
        }
        result.push(tmp.iter().any(|x1| {*x1 == true}))
    }

    let b_set: HashSet<i64> = b.clone().into_iter().collect();
    for b_i in  &b_set {
        for a_i in &a {
            let b_key = (*b_i as f64).sqrt() as i64;
            let n_b = match b_vs_a_dict.get(&b_key) {
                Some(v) => *v,
                None => 0,
            };

            if a_i.pow(2) == *b_i {
                b_vs_a_dict.insert(b_key, n_b + 1);
            }
        }
    }

    if a_vs_b_dict == b_vs_a_dict {
        result.iter().all(|x| {*x == true})
    }
    else {
        false
    }
}

// 模範回答
/*
fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut a1 = a.iter().map(|&x| x*x).collect::<Vec<_>>();
    let mut a2 = b;
    a1.sort();
    a2.sort();

    a1 == a2
}
*/

fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
    assert_eq!(comp(a, b), exp)
}

#[test]
fn tests_comp() {

    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    // 121 ... 1
    // 144 ... 2
    // 19 ... 3
    // 161 ... 1
    // 11 ... 1
    // --
    // 11*11 ... 1
    // 121*121 ... 1
    // 144*144 ... 2
    // 19*19 ... 3
    // 161*161 ... 1
    testing(a1, a2, true);
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*21, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    testing(a1, a2, false);
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![121, 14641, 20736, 36100, 25921, 361, 20736, 361];
    testing(a1, a2, false);
    let a1 = vec![2, 2, 3];
    let a2 = vec![4, 9, 9];
    // 2 ... 1
    // 3 ... 2
    // ---
    // 4 ... 2
    // 9 ... 1
    testing(a1, a2, false);
}