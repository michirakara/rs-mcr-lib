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

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 与えられた関数でDFSを実行する
    ///
    /// - `init` は `(index, data)` のタプル
    /// - `pre_order(&idx, &nidx, &cost, &data) -> Option<data>` で行きがけの処理
    ///     - 返り値が `Some(data)` なら `nidx` に対しても探索を行う
    ///     - 返り値が `None` なら `nidx` に対しては探索は行わない
    ///     - `dfs` 側では既に探索したかの判定は **行わない** ので `pre_order` 内で判定をすること
    /// - `post_order(&idx, &data)` で帰りがけの処理
    fn dfs<D: Clone, F, G>(&self, init: (Self::Idx, D), mut pre_order: F, mut post_order: G)
    where
        F: FnMut(&Self::Idx, &Self::Idx, &Self::Cost, &D) -> Option<D>,
        G: FnMut(&Self::Idx, &D),
    {
        let mut stk = vec![
            (Dfs::Post(init.0.clone()), init.1.clone()),
            (Dfs::Pre(init.0.clone()), init.1.clone()),
        ];
        while let Some((idx, data)) = stk.pop() {
            match idx {
                Dfs::Pre(idx) => {
                    for (nidx, cost) in self.edges(idx.clone()) {
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
#[derive(Debug, Clone)]
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
}
#[derive(Debug, Clone)]
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
}
#[derive(Debug, Clone)]
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
        if !self.g[i.0][i.1] {
            return Vec::new();
        }

        let (dx, dy) = if self.is_8way {
            (
                vec![-1, -1, -1, 0, 0, 1, 1, 1],
                vec![-1, 0, 1, -1, 1, -1, 0, 1],
            )
        } else {
            (vec![-1, 0, 0, 1], vec![0, -1, 1, 0])
        };

        let mut ret = vec![];
        for (&dx, &dy) in dx.iter().zip(dy.iter()) {
            let nx: Option<usize> = ((i.0 as i32) + dx).try_into().ok();
            let ny: Option<usize> = ((i.1 as i32) + dy).try_into().ok();
            if let (Some(nx), Some(ny)) = (nx, ny) {
                if nx < self.h && ny < self.w && self.g[nx][ny] {
                    ret.push(((nx, ny), 1));
                }
            }
        }
        ret
    }
}
