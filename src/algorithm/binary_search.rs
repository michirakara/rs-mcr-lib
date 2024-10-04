use num::traits::{FromPrimitive, Num};

/// `ok` と `ng` の差が `threshold` 以下になるまで二分探索
/// オーバーフローしない
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

/// `trial` 回二分探索
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
