use num::traits::{FromPrimitive, Num};

pub fn binary_search_threshold<T, F>(mut ok: T, mut ng: T, mut check: F, threshold: T) -> T
where
    T: Num + PartialOrd + FromPrimitive + Copy,
    F: FnMut(T) -> bool,
{
    while if ok > ng { ok - ng } else { ng - ok } > threshold {
        let mid = if ok > ng {
            ng + (ok - ng) / T::from_usize(2).unwrap()
        } else {
            ok + (ng - ok) / T::from_usize(2).unwrap()
        };
        if check(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

pub fn binary_search_trial<T, F>(mut ok: T, mut ng: T, mut check: F, trial: usize) -> T
where
    T: Num + PartialOrd + FromPrimitive + Copy,
    F: FnMut(T) -> bool,
{
    for _ in 0..trial {
        let mid = if ok > ng {
            ng + (ok - ng) / T::from_usize(2).unwrap()
        } else {
            ok + (ng - ok) / T::from_usize(2).unwrap()
        };
        if check(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
