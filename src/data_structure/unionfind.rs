use super::super::math::algebra::Group;

pub struct UnionFind {
    par: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.find(self.par[x]);
        self.par[x]
    }

    pub fn merge(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }
        if self.size[x] > self.size[y] {
            self.size[x] += self.size[y];
            self.par[y] = self.par[x];
        } else {
            self.size[y] += self.size[x];
            self.par[x] = self.par[y];
        }
    }

    pub fn size(&mut self, x: usize) -> usize {
        let x = self.find(x);
        self.size[x]
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

pub struct PotentializedUnionFind<T: Group>
where
    T::Set: Clone,
{
    par: Vec<usize>,
    size: Vec<usize>,
    diff_weight: Vec<T::Set>,
}
impl<T: Group> PotentializedUnionFind<T>
where
    T::Set: Clone,
{
    pub fn new(n: usize) -> Self {
        PotentializedUnionFind::<T> {
            par: (0..n).collect(),
            size: vec![1; n],
            diff_weight: vec![T::id(); n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        let tmp = self.find(self.par[x]);
        self.diff_weight[x] = T::op(
            self.diff_weight[x].clone(),
            self.diff_weight[self.par[x]].clone(),
        );
        self.par[x] = tmp;
        self.par[x]
    }

    pub fn weight(&mut self, x: usize) -> T::Set {
        self.find(x);
        self.diff_weight[x].clone()
    }
    /// `weight(x) + y == weight(y)`
    pub fn merge(&mut self, x: usize, y: usize, mut w: T::Set) -> bool {
        w = T::op(w, self.weight(x));
        w = T::op(w, T::inv(self.weight(y)));
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return false;
        }
        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
            w = T::inv(w);
        }
        self.size[x] += self.size[y];
        self.par[y] = self.par[x];
        self.diff_weight[y] = w;
        true
    }

    pub fn size(&mut self, x: usize) -> usize {
        let x = self.find(x);
        self.size[x]
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    /// `Some(weight(y) - weight(x))`
    /// 繋がってないなら `None`
    pub fn diff(&mut self, x: usize, y: usize) -> Option<T::Set> {
        if self.same(x, y) {
            Some(T::op(self.weight(y), T::inv(self.weight(x))))
        } else {
            None
        }
    }
}
