extern crate uvector;

use std::collections::VecDeque;
use uvector::UVec;

fn check_sum(vd: &VecDeque<i32>, exp: i32) {
    let uv = UVec::new(vd.as_slices());
    let sum = uv.iter().fold(0, |sum, x| sum + x);
    assert_eq!(sum, exp);
}

#[test]
fn vecdeque() {
    let mut vd: VecDeque<i32> = VecDeque::new();
    for i in 1i32..6 {
        vd.push_back(i);
    }
    for _ in 1..4 {
        vd.pop_front();
    }
    vd.push_back(6);
    check_sum(&vd, 15);
    vd.push_back(7);
    check_sum(&vd, 22);
}
