use std::iter::zip;

#[derive(Clone, Debug)]
enum Dfs<Idx> {
    Pre(Idx),
    Post(Idx),
}

pub trait Graph {
    type Idx: Clone;
    type Cost;

    fn len(&self) -> usize;
    fn edges(&self, i: Self::Idx) -> Vec<(Self::Idx, Self::Cost)>;
    fn idx_to_usize(&self, i: Self::Idx) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn dfs<D: Clone, F, G>(&self, init: (Self::Idx, D), mut pre_order: F, mut post_order: G)
    where
        F: FnMut(&Self::Idx, &Self::Idx, &Self::Cost, &D) -> Option<D>,
        G: FnMut(&Self::Idx, &D),
    {
        let mut done = vec![false; self.len()];
        let mut stk = vec![
            (Dfs::Post(init.0.clone()), init.1.clone()),
            (Dfs::Pre(init.0.clone()), init.1.clone()),
        ];
        while let Some((idx, data)) = stk.pop() {
            match idx {
                Dfs::Pre(idx) => {
                    done[self.idx_to_usize(idx.clone())] = true;
                    for (nidx, cost) in self.edges(idx.clone()) {
                        if done[self.idx_to_usize(nidx.clone())] {
                            continue;
                        }
                        let ndata = pre_order(&idx, &nidx, &cost, &data);
                        if let Some(ndata) = ndata {
                            stk.push((Dfs::Post(nidx.clone()), ndata.clone()));
                            stk.push((Dfs::Pre(nidx.clone()), ndata.clone()));
                        }
                    }
                }
                Dfs::Post(idx) => {
                    post_order(&idx, &data);
                }
            }
        }
    }
}
#[derive(Debug)]
pub struct UnweightedGraph {
    pub n: usize,
    pub g: Vec<Vec<usize>>,
}

impl Graph for UnweightedGraph {
    type Idx = usize;
    type Cost = usize;

    fn len(&self) -> usize {
        self.n
    }

    fn edges(&self, i: Self::Idx) -> Vec<(Self::Idx, Self::Cost)> {
        self.g[i].iter().map(|&i| (i, 1)).collect()
    }

    fn idx_to_usize(&self, i: Self::Idx) -> usize {
        i
    }
}
#[derive(Debug)]
pub struct WeightedGraph<Cost> {
    pub n: usize,
    pub g: Vec<Vec<(usize, Cost)>>,
}

impl<Cost: Clone> Graph for WeightedGraph<Cost> {
    type Idx = usize;
    type Cost = Cost;

    fn len(&self) -> usize {
        self.n
    }

    fn edges(&self, i: Self::Idx) -> Vec<(Self::Idx, Self::Cost)> {
        self.g[i].clone()
    }

    fn idx_to_usize(&self, i: Self::Idx) -> usize {
        i
    }
}
#[derive(Debug)]
pub struct Grid {
    pub is_8way: bool,
    pub h: usize,
    pub w: usize,
    pub g: Vec<Vec<bool>>,
}

impl Graph for Grid {
    type Idx = (usize, usize);
    type Cost = usize;

    fn len(&self) -> usize {
        self.h * self.w
    }

    fn edges(&self, i: Self::Idx) -> Vec<(Self::Idx, Self::Cost)> {
        if self.is_8way {
            const DX: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
            const DY: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
            let mut ret = vec![];
            for (dx, dy) in zip(DX, DY) {
                let nx: Option<usize> = ((i.0 as i32) + dx).try_into().ok();
                let ny: Option<usize> = ((i.1 as i32) + dy).try_into().ok();
                if let Some(nx) = nx {
                    if let Some(ny) = ny {
                        if nx < self.h && ny < self.w && !self.g[nx][ny] {
                            ret.push(((nx, ny), 1));
                        }
                    }
                }
            }
            ret
        } else {
            const DX: [i32; 4] = [-1, 0, 0, 1];
            const DY: [i32; 4] = [0, -1, 1, 0];
            let mut ret = vec![];
            for (dx, dy) in zip(DX, DY) {
                let nx: Option<usize> = ((i.0 as i32) + dx).try_into().ok();
                let ny: Option<usize> = ((i.1 as i32) + dy).try_into().ok();
                if let Some(nx) = nx {
                    if let Some(ny) = ny {
                        if nx < self.h && ny < self.w && !self.g[nx][ny] {
                            ret.push(((nx, ny), 1));
                        }
                    }
                }
            }
            ret
        }
    }

    fn idx_to_usize(&self, i: Self::Idx) -> usize {
        i.0 * self.h + i.1
    }
}
