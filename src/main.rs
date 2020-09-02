#![allow(unused)]
use std::time::Instant;

mod bin_heap;
mod quad_heap;

type HEAP<T> = quad_heap::QuadHeap<T>;

fn main() {
    let rng = fastrand::Rng::with_seed(92);

    let n = 1 << 20;
    let mut heap = HEAP::with_capacity(n);
    {
        let t = Instant::now();
        for _ in 0..n {
            heap.push(rng.u64(..))
        }
        eprintln!("push: {:.2?}", t.elapsed());
    }
    let mut res = Vec::with_capacity(n);
    {
        let t = Instant::now();
        while let Some(elt) = heap.pop_min() {
            res.push(elt)
        }
        eprintln!("pop: {:.2?}", t.elapsed());
    }

    let mut res2 = res.clone();
    res2.sort();
    assert_eq!(res, res2);
}
