A Rust library providing read-only array type allowing access two slices as a
single continuous vector. Can be used for example to conveniently access
parts of VecDeque.

# Example

```rust
use std::collections::VecDeque;
use uvector::UVec;

// Return sum of the first 3 numbers in VecDeque
fn head3_sum(vd: &VecDeque<i32>) -> i32 {
    let uv = UVec::new(vd.as_slices());
    uv.range(0, 3).iter().fold(0, |sum, x| sum + x)
}

fn main() {
    let mut vd: VecDeque<i32> = VecDeque::new();
    for i in 1..6 {
        vd.push_back(i);
    }
    let s = head3_sum(&vd);
    assert_eq!(s, 6);
}
```
